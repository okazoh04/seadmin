# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

**seadmin** は SELinux のアクセス拒否イベントを管理する Rust 製の TUI（テキストユーザーインターフェース）ツールです。`ausearch` の出力を解析し、修復策を提示・実行します。

## Build & Development Commands

```bash
cargo build              # 開発ビルド
cargo build --release    # リリースビルド（strip + LTO + サイズ最適化）
cargo test               # テスト実行
cargo clippy             # リント
cargo fmt                # フォーマット
cargo test <test_name>   # 特定テストのみ実行
```

インストール:
```bash
sudo install -m 755 target/release/seadmin /usr/local/bin/
```

リリースビルドのデフォルトターゲットは `x86_64-unknown-linux-musl`（self-contained バイナリ）。

## Architecture

### データフロー

```
ausearch → AVC エントリ解析 → (process, action, target) で集約
→ 修復策の提示 → TUI 表示 → ユーザー操作
→ sudo コマンド実行 → ログ記録
```

### モジュール構成

**`src/main.rs`** — エントリーポイント。tokio イベントループ、TUI 初期化、非同期タスク管理。

**`src/selinux/`**
- `avc.rs`: ausearch 出力のパース、`AvcEntry` 構造体の定義、修復策の種類（`Restorecon`, `FileContext`, `PortContext`, `Boolean`, `CustomPolicy`）
- `commands.rs`: ausearch / getenforce / semanage / audit2allow / restorecon / setsebool / semodule 等のラッパー

**`src/sudo/runner.rs`** — sudo パスワードのセキュアなキャッシュ管理。`zeroize` クレートでメモリを消去。パスワードはプロセス引数ではなく stdin 経由で渡す。3 回失敗で 60 秒ロックアウト。

**`src/ui/`**
- `app.rs`: アプリ全体の状態管理・画面遷移・イベントハンドリング
- `screens/avc_list.rs`: 集約されたアクセス拒否一覧画面（メイン画面）
- `screens/avc_detail.rs`: 詳細・修復オプション選択画面
- `screens/auth_popup.rs`: sudo パスワード入力ポップアップ
- `screens/policy_review.rs`: audit2allow ポリシーのレビュー・適用画面
- `widgets.rs`: ヘッダー・フッター・ステータスバー・ログオーバーレイ等の共通 Widget

**`src/i18n/`** — マクロベースの多言語対応（18 言語）。`LANG` 環境変数で自動検出。各言語は独立したファイル（`ja.rs`, `en.rs`, 等）で管理。

### 外部依存コマンド

実行環境に以下が必要: `ausearch`, `getenforce`, `sudo`, `semanage`, `audit2allow`, `restorecon`, `setsebool`, `semodule`

## i18n の追加・修正

新しい文字列を追加する場合:
1. `src/i18n/mod.rs` のマクロ定義に追加
2. 各言語ファイル（`src/i18n/*.rs`）に対応する翻訳を追加
3. 全 18 言語ファイルへの追加が必要
