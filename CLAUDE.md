# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`seadmin` は SELinux の AVC デナイアルを TUI で確認・対処するための Rust 製管理ツール。

## コマンド

```bash
# ビルド
cargo build

# リリースビルド（strip + LTO + サイズ最適化 済み）
cargo build --release

# 実行
cargo run

# テスト
cargo test

# 単一テストの実行
cargo test <テスト名>

# Lint
cargo clippy

# フォーマット
cargo fmt
```

実行には `ausearch`、`getenforce`、`sudo` などの SELinux ツールが必要。`LANG=ja_JP.UTF-8` 推奨（起動時にチェックあり）。

## アーキテクチャ

### 非同期メッセージパッシング

`main.rs` の `run_app` がメインループ。バックグラウンドタスク（`tokio::spawn`）と UI スレッドは `tokio::sync::mpsc` チャンネル（`AppMsg` enum）で通信する。UI スレッドはチャンネルをポーリングして状態を更新し、50ms ごとに再描画する。

### 画面遷移

`App::screen_stack: Vec<Screen>` によるスタック管理。`push_screen` / `pop_screen` で遷移し、`current_screen()` で現在画面を参照する。Auth 画面は前画面を背景として重ねて描画する仕組み（`AuthContext::prev_screen` に戻り先を保持）。

### モジュール構成

| パス | 役割 |
|------|------|
| `src/main.rs` | エントリポイント、イベントループ、`AppMsg` 処理 |
| `src/selinux/avc.rs` | `AvcEntry` 構造体、`ausearch` 出力パーサ、`Remedy` 判定 |
| `src/selinux/commands.rs` | システムコマンドの async ラッパー（`ausearch`, `getenforce`, `getsebool`, `audit2allow` 等） |
| `src/sudo/runner.rs` | `sudo -S -k` 経由のコマンド実行、`SudoResult` |
| `src/ui/app.rs` | `App`（全状態）、`Screen`、`AuthState`（ロックアウト管理） |
| `src/ui/screens/avc_list.rs` | AVC 一覧画面のレンダラ |
| `src/ui/screens/avc_detail.rs` | AVC 詳細・対処オプション選択画面 |
| `src/ui/screens/auth_popup.rs` | sudo パスワード入力ポップアップ |
| `src/ui/widgets.rs` | ヘッダ・フッタ・ステータスバーなど共通ウィジェット |

### セキュリティ上の注意

パスワードバッファは `Zeroizing<String>`（`zeroize` クレート）で管理し、`drop` 時にメモリがゼロ化される。`sudo::runner` はパスワードを stdin に書き込み後すぐに drop する。認証失敗 3 回で 60 秒ロックアウト（`AuthState::on_fail`）。

### AVC 集計ロジック

`parse_ausearch_output` は `scontext|tcontext|tclass|perm` をキーに重複を集計し、`last_seen` で降順ソートして ID を振り直す。`ausearch` は一致なしでも exit code 1 を返すため、成否判定は stderr の "Permission denied" 有無で行う。

## 多言語対応（i18n）

### 仕組み

`src/i18n/mod.rs` の `define_langs!` マクロで言語テーブルを管理。`LANG` / `LC_ALL` / `LC_MESSAGES` 環境変数でロケールを検出し、`detect_lang()` が `Lang` enum を返す。

### ルール：ユーザーに見える文字列は必ず i18n を使う

**UI テキスト、ログ出力、エラーメッセージを含む、ユーザーが目にする可能性のある全ての文字列**を直接ハードコードしてはならない。必ず対応する `Lang` メソッドを経由すること。

| 場所 | アクセス方法 |
|------|------------|
| UI レンダラ・イベントループ | `app.lang.xxx()` |
| `check_deps()` など `Lang` を持たない関数 | `crate::i18n::detect_lang()` でインスタンスを取得 |
| `commands.rs` などの async 関数 | `crate::i18n::detect_lang()` をローカルで呼ぶ |

### 新しいテキストを追加する手順

1. `src/i18n/mod.rs` の `define_langs!` マクロ内に新しいメソッドを追加
2. `src/i18n/ja.rs` に日本語テキストを追加
3. `src/i18n/en.rs` に英語テキストを追加
4. 残り16言語ファイルにも対応テキストを追加（`src/i18n/` 以下の全 `.rs` ファイル）
5. 呼び出し元で `lang.新メソッド()` を使用

### 言語ファイル一覧

`src/i18n/` に ja, en, zh_hans, zh_hant, ko, ru, kk, es, pt, fr, de, it, nl, sv, no, ar, th, vi の18言語がある。新言語追加は `mod.rs` の `define_langs!` に1行追加するだけ。
