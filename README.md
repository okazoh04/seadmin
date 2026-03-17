# seadmin

[English](README.en.md) | 日本語 | [中文](README.zh.md)

SELinux のアクセス拒否を TUI（テキストユーザーインターフェース）で確認・対処するための Rust 製管理ツール。

## 概要

SELinux の `ausearch` 出力を解析し、アクセス拒否の一覧表示・原因分析・対処コマンドの実行を対話的に行えます。

- アクセス拒否の集計・一覧表示（プロセス / 操作 / 対象 / 発生件数）
- 原因分析と解決策の自動候補提示
- `restorecon` / `semanage fcontext` / `setsebool` / `audit2allow` による対処をそのまま実行
- sudo パスワードをセッション内でキャッシュし、複数操作でも再入力を最小化
- 操作ログを `~/.local/share/seadmin/seadmin.log` に記録
- **多言語対応**（日本語 / 英語 / 中国語）— `LANG` 環境変数で自動切替

## スクリーンショット

```
┌─ アクセス拒否一覧  [本日]  未対処: 3件 / 全 3件 ──────────────────────────┐
│ #  発生      プロセス      操作          対象                   件数 解決策候補  │
│▶1  3分前     nginx         name_bind     /var/run/nginx.sock      2  ポート追加  │
│ 2  1時間前   httpd         write         /var/www/html/upload     5  fcontext変更│
│ 3  2日前     mysqld_t      name_connect  192.168.1.100:3306       1  Boolean:... │
└────────────────────────────────────────────────────────────────────────────────┘
[seadmin] [Enforcing] hostname  ↑↓/jk:移動  Enter:詳細  /:フィルタ  r:更新  q:終了
```

## 必要環境

- Linux（SELinux が有効なディストリビューション: Fedora / RHEL / CentOS / Rocky Linux など）
- Rust 1.85 以上（edition 2024）
- 以下のコマンドが `PATH` に存在すること:
  - `ausearch`（audit パッケージ）
  - `getenforce`（libselinux-utils）
  - `sudo`
  - `semanage`（policycoreutils-python-utils）※対処操作に使用
  - `audit2allow`（policycoreutils-devel）※カスタムポリシー生成に使用
  - `restorecon`（policycoreutils）※ラベル修復に使用
  - `setsebool`（libselinux-utils）※Boolean 変更に使用
  - `semodule`（policycoreutils）※ポリシーモジュール適用に使用

## インストール

```bash
git clone https://github.com/yourusername/seadmin.git
cd seadmin

# リリースビルド（strip + LTO + サイズ最適化済み）
cargo build --release

# バイナリをパスの通った場所に配置
sudo install -m 755 target/release/seadmin /usr/local/bin/
```

## 使い方

```bash
seadmin
```

`ausearch` の実行に root 権限が必要な環境では:

```bash
sudo seadmin
```

### 言語切替

`LANG` 環境変数の先頭部分で表示言語が自動決定されます。

| LANG の値 | 表示言語 |
|-----------|----------|
| `ja_*` | 日本語（デフォルト） |
| `zh_*` | 中国語（簡体字） |
| その他 | 英語 |

## キーバインド

### アクセス拒否一覧画面

| キー | 操作 |
|------|------|
| `↑` / `k` | カーソル上移動 |
| `↓` / `j` | カーソル下移動 |
| `Enter` | 詳細・対処画面を開く |
| `/` | フィルタ入力開始（プロセス名 / 操作 / 対象で絞り込み） |
| `Esc` | フィルタをクリア |
| `r` | アクセス拒否ログを再取得 |
| `l` | 操作ログオーバーレイを表示 / 非表示 |
| `q` | 終了 |
| `Ctrl+C` | 強制終了 |

### 詳細・対処画面

| キー | 操作 |
|------|------|
| `↑` / `k` | オプション選択を上移動 |
| `↓` / `j` | オプション選択を下移動 |
| `A`〜`F` | 対処オプションをキー直接選択 |
| `Enter` | 選択した対処を実行（sudo 認証ポップアップ） |
| `Esc` / `←` | 一覧画面に戻る |

### ポリシーレビュー画面（audit2allow）

| キー | 操作 |
|------|------|
| `↑` / `k` | スクロール上 |
| `↓` / `j` | スクロール下 |
| `Enter` | ポリシーをシステムに適用 |
| `Esc` | キャンセル（生成ファイルを削除） |

## 対処オプションの種類

| 解決策候補 | 内容 |
|------------|------|
| **ポート追加** | `semanage port -a` でポートコンテキストを追加 |
| **fcontext 変更** | `semanage fcontext -a` でファイルコンテキストルールを追加 |
| **restorecon** | `restorecon -Rv` でデフォルトコンテキストに修復 |
| **Boolean** | `setsebool` で SELinux Boolean を有効化（一時 / 永続） |
| **カスタムポリシー** | `audit2allow` でポリシーモジュールを自動生成・レビュー・適用 |
| **Permissive 設定** | `semanage permissive -a` で対象ドメインを一時的に Permissive 化（調査用） |

## セキュリティ

- sudo パスワードは [`zeroize`](https://docs.rs/zeroize) クレートで管理し、`drop` 時にメモリがゼロ化されます
- 認証失敗 3 回で 60 秒のロックアウトが発生します
- パスワードは stdin 経由で `sudo -S -k` に渡し、プロセス引数には含まれません

## 開発

```bash
# 開発ビルド
cargo build

# テスト
cargo test

# Lint
cargo clippy

# フォーマット
cargo fmt
```

## ライセンス

GPL-3.0
