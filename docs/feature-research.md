# seadmin 機能拡張計画

> 作成日: 2026-03-18
> 最終更新: 2026-03-20（v0.4.0 時点の実装状況を反映）
> 参考ツール: setroubleshoot/sealert、audit2allow/audit2why、policycoreutils-gui、sepolicy

---

## 1. 参考実装の調査

### 1.1 setroubleshoot / sealert

バックグラウンドデーモン `setroubleshootd` が audit サブシステムを監視し、新規 AVC をリアルタイムで 51 本の Python プラグインにかけて解析・SQLite に蓄積する。`sealert` はその CLI/GUI フロントエンド。

**集約キー（seadmin との差異）：**

| | seadmin | setroubleshoot |
|---|---|---|
| キー構成 | `scontext\|tcontext\|tclass\|perm` | `source_comm\|scontext.type\|tcontext.type\|tclass\|sorted(perms)` |
| 特徴 | プロセス名を含まない | 同じ type ペアでも comm 違いは別グループ |

**追加解析フィールド（seadmin に未実装）：**

| フィールド | 内容 |
|---|---|
| `pid` / `exe` / `cwd` | プロセス情報 |
| `arch` / `syscall` | アーキテクチャ、システムコール名（番号→名前変換済み） |
| `success` / `exit` | 成否フラグ、errno 名（`EACCES` 等） |
| `src` / `dest` | ネットワークポート番号 |
| `bools` | この拒否に関連する SELinux boolean 名（`audit2why` から抽出） |
| `src_rpms` / `tgt_rpms` | ソース・ターゲットの RPM パッケージ名 |
| `host` | ホスト名 |
| `firstSeen` | 初回発生タイムスタンプ |

**主要プラグイン一覧：**

| プラグイン | 優先度 | レベル | 修復コマンド |
|---|---|---|---|
| restorecon.py | 100 | 緑 | `restorecon -Rv PATH`（matchpathcon でラベル不正を検出） |
| file.py (unlabeled) | 70 / 30 | 緑 | `restorecon -R -v PATH` または `touch /.autorelabel; reboot` |
| catchall_boolean.py | 8 | 黄 | `setsebool -P BOOL on`（`avc.bools` から動的取得） |
| allow_execmem.py | 10 | 赤 | boolean が存在すれば setsebool、なければ管理者エスカレーション |
| allow_execstack.py | — | — | `execstack -c LIBRARY_PATH` |
| catchall_labels.py | 5 | — | `semanage fcontext -a -t TYPE PATH; restorecon -v PATH` |
| catchall.py | 1 | 黄 | `ausearch -c COMM --raw \| audit2allow -M MODULE; semodule -X 300 -i MODULE.pp` |

プラグインは優先度降順で全件マッチングを行い、ヒットした全結果を UI に表示（優先度が高い = より具体的な修復策）。

---

### 1.2 audit2allow / audit2why

**audit2allow の出力モード：**

| オプション | 出力形式 |
|---|---|
| なし | 生 TE allow ルール |
| `-M NAME` | コンパイル済み `.pp` モジュールパッケージ |
| `-C` | CIL 構文 |
| `-R` | refpolicy スタイル（インターフェースマクロ使用） |
| `-D` | `dontaudit` ルール |
| `-x` | ioctl allowxperm ルール |

**audit2why モード（`-w`）：**
libsepol でポリシー本体に問い合わせ、拒否の根本原因をカテゴリで返す：

| 返値 | 意味 | 推奨対処 |
|---|---|---|
| `TERULE` | TE allow ルール不足 | audit2allow でモジュール生成 |
| `BOOLEAN` | boolean が off | `setsebool -P BOOL on` |
| `BADTCON` | ターゲットのラベル不正 | `restorecon` |
| `CONSTRAINT` | MLS/MCS 制約違反 | ポリシー設計の問題 |
| `DONTAUDIT` | dontaudit で隠蔽されている | 別調査が必要 |
| `ALLOW` | allow ルールは存在（別要因） | 別調査が必要 |

現在の seadmin `diagnose_remedy()` はヒューリスティック推定だが、audit2why を使うとポリシー本体への正確な問い合わせになる。

**semodule の優先度：**
ローカルカスタムポリシーは `-X 300` を指定して優先度 300 で読み込むのが標準（base ポリシー = 100 より高く確実に適用される）。現在の seadmin は `-X` 指定なし。

---

### 1.3 policycoreutils-gui / sepolicy gui

GTK アプリ `system-config-selinux`（`sepolicy gui` でも起動可）の機能：

- SELinux boolean の一覧表示・トグル（説明文付き）
- ファイルコンテキストの閲覧・デフォルトコンテキスト検索
- ユーザー/ロールマッピング管理
- ネットワークポート型アサインの閲覧
- **ポリシーモジュール管理**（一覧・インストール・削除）
- SELinux モード切り替え
- プロセスドメインとドメイン遷移の閲覧

---

### 1.4 sepolicy コマンド

| サブコマンド | 機能 |
|---|---|
| `booleans -b BOOL` | boolean の人間可読な説明文を表示 |
| `communicate -s SRC -t TGT` | 2 ドメイン間の通信可否を判定 |
| `generate` | 実行バイナリからポリシーモジュール雛形を生成 |
| `network -d DOMAIN_T` | ドメインがバインド/接続できるポート型一覧 |
| `network -a /path/to/bin` | パスからポート情報を検索 |
| `transition -s SRC` | SRC が遷移できるドメイン一覧（エントリポイント付き） |
| `interface` | ポリシーインターフェース一覧 |
| `manpage` | ドメイン型の man ページ自動生成 |

---

## 2. 改善ポイント

### 2.1 表示情報のギャップ

| フィールド | 他ツール（setroubleshoot 等） | seadmin 現状 |
|---|---|---|
| last_seen | あり | あり |
| first_seen | あり | **なし** |
| severity レベル（緑/黄/赤） | あり | **なし** |
| 関連 boolean 名（audit2why 由来） | あり | **なし** |
| syscall 名（番号→名前変換） | あり | **なし** |
| errno 名 | あり | **なし** |
| RPM パッケージ名 | あり | **なし** |

### 2.2 修復アクションのギャップ

| シナリオ | setroubleshoot | seadmin 現状 |
|---|---|---|
| restorecon（ラベル不正） | 優先度 100 で最初に提示 | unlabeled_t 限定 |
| 全体再ラベル（`.autorelabel`） | 優先度 30 で提示 | **なし** |
| semanage fcontext + restorecon 2ステップ | catchall_labels.py | semanage のみで restorecon なし |
| 動的 boolean 検出 | audit2why から自動取得 | 4 エントリ hardcode のみ |
| audit2why による根本原因判定 | 全 AVC に適用 | **なし（ヒューリスティックのみ）** |
| execstack / execmem | 専用プラグイン | Remedy enum に未定義 |
| 複数修復案を優先度順に提示 | 全マッチプラグイン表示 | 単一 Remedy のみ |
| semodule -X 300（優先度指定） | catchall.py で実施 | **-X 指定なし** |

### 2.3 フィルタ / 検索

- sealert: ログファイル・時間範囲フィルタのみ（`-a FILE`）
- seadmin: `/` キーでプロセス名・パス・パーミッションによるテキストフィルタあり → **既にアドバンテージあり**

---

## 3. 機能拡張ロードマップ

### 高優先度

#### A. `audit2why` 統合による原因診断精度向上　🔲 未実装

**概要：** 現在の `diagnose_remedy()` をヒューリスティックから `audit2why`（`audit2allow -w`）呼び出しに置き換える。

**実装イメージ：**
```sh
ausearch の raw 出力 | audit2allow -w
```
出力から `TERULE` / `BOOLEAN` / `BADTCON` / `CONSTRAINT` を解析して `Remedy` を決定。

**効果：** 誤診断の大幅削減、関連 boolean 名の動的取得

**難易度：** 中

---

#### B. 動的 Boolean 検出　🔲 未実装

**概要：** `BOOLEAN_MAP`（現在 4 エントリ）を廃止し、`audit2why` 出力から boolean 名を抽出。

**実装イメージ：** audit2why 出力の `"If you want to allow ... you must turn on the ... boolean"` パターンを正規表現で抽出。

**効果：** httpd/ftpd/samba 以外のサービスでも Boolean remedy が機能する

**難易度：** 小

---

#### C. severity レベル表示（緑/黄/赤）　✅ 実装済み（v0.4.3）

**概要：** AVC 一覧の Remedy 列またはインジケータ列に色付きレベルを表示。

| レベル | 条件 |
|---|---|
| 緑 | `BADTCON` → restorecon で修復可能 |
| 黄 | `BOOLEAN` / `PortContext` / `FileContext` |
| 赤 | `CustomPolicy`（未知の拒否）、または `CONSTRAINT` |

**難易度：** 小

---

### 中優先度

#### D. `semanage fcontext` + `restorecon` 2ステップ修復　✅ 実装済み

**概要：** `FileContext` remedy の修復コマンドを 2 ステップに修正。

```sh
semanage fcontext -a -t TYPE 'PATH(/.*)?'; restorecon -Rv PATH
```

`sh -c` で一括実行するよう実装済み。

**難易度：** 小

---

#### E. `semodule -X 300` の適用　✅ 実装済み（v0.4.3）

**概要：** カスタムポリシーのインストール時に優先度 300 を指定する。

**現状：** `semodule -X 300 -i NAME.pp`

**修正後：** `semodule -X 300 -i NAME.pp`

**難易度：** 極小（1 行変更）

---

#### F. first_seen タイムスタンプの表示　✅ 実装済み

**概要：** ausearch 出力の最初のレコードのタイムスタンプを `first_seen` として保存し、詳細画面に「初回発生 / 最終発生」を両表示。複数回発生時のみ表示。

**難易度：** 小

---

#### G. Boolean 説明文の表示　✅ 実装済み

**概要：** Boolean remedy 時に `sepolicy booleans -b BOOLNAME` を呼び出し、その boolean を有効にすると何が許可されるかの説明文を詳細画面に表示。`AvcEntry::bool_description` フィールドに非同期で補完。

**難易度：** 小

---

#### H. ポリシーモジュール管理画面　✅ 実装済み（v0.4.0）

**概要：** `semodule -lfull` の出力一覧を表示する新規画面。`semodule -X <priority> -r <name>` で削除可能。AVC 一覧から `m` キーで遷移。優先度降順ソート。

**難易度：** 大

---

### 低優先度

#### I. `sepolicy network` によるポート情報補完　🔲 未実装

**概要：** PortContext remedy 時に `semanage port --list` または `sepolicy network -d DOMAIN_T` を呼び出し、そのドメインが本来利用できるポート型の一覧を詳細画面に表示。管理者が適切な port type を自分で選択できるようになる。

**難易度：** 中

---

#### J. syscall 名・errno 名の変換表示　✅ 実装済み（v0.4.0）

**概要：** audit ログの `syscall=NNN` を `read` / `open` / `bind` 等のシンボル名に変換、`exit=-13` を `EACCES` 等に変換して詳細画面に表示。x86_64 syscall テーブル（~70 エントリ）と Linux errno テーブル（~40 エントリ）を静的テーブルで実装。

**難易度：** 小

---

#### K. 全ファイルシステム再ラベル（`.autorelabel`）　🔲 未実装

**概要：** `unlabeled_t` が大量発生している場合に限り `touch /.autorelabel && reboot` の選択肢を提示。

**難易度：** 小

---

## 4. 優先度マトリクス

| # | 機能 | 難易度 | 効果 | 状態 |
|---|---|---|---|---|
| A | audit2why 統合による原因診断精度向上 | 中 | 大 | 🔲 未実装 |
| B | 動的 Boolean 検出 | 小 | 大 | 🔲 未実装 |
| E | semodule -X 300 適用 | 極小 | 中 | 🔲 未実装 |
| D | semanage fcontext + restorecon 2ステップ修復 | 小 | 中 | ✅ 実装済み |
| C | severity レベル表示（緑/黄/赤） | 小 | 中 | 🔲 未実装 |
| F | first_seen タイムスタンプ表示 | 小 | 中 | ✅ 実装済み |
| G | Boolean 説明文表示 | 小 | 中 | ✅ 実装済み |
| H | ポリシーモジュール管理画面 | 大 | 中 | ✅ 実装済み（v0.4.0） |
| I | sepolicy network によるポート情報補完 | 中 | 小 | 🔲 未実装 |
| J | syscall / errno 名変換表示 | 小 | 小 | ✅ 実装済み（v0.4.0） |
| K | 全ファイルシステム再ラベル | 小 | 小 | 🔲 未実装 |

---

## 5. 現在の seadmin 機能サマリ（v0.4.0）

### 画面構成

| 画面 | 概要 |
|---|---|
| AVC 一覧 | ausearch 結果を集約表示、`/` でフィルタ、`m` でモジュール管理へ |
| AVC 詳細 | 原因分析（syscall/errno 表示含む）・修復オプション選択・生ログ表示 |
| Policy Review | audit2allow 生成の .te ファイル表示・適用 |
| Module List | semodule -lfull 一覧、`d` キーで削除（v0.4.0 追加） |
| Auth Popup | sudo パスワード入力モーダル（3 回失敗で 60 秒ロックアウト） |

### 実行コマンド

| コマンド | 用途 |
|---|---|
| `ausearch -m AVC,USER_AVC,SELINUX_ERR -ts today` | AVC ログ取得 |
| `getenforce` | SELinux モード取得 |
| `audit2allow -M NAME` | カスタムポリシー生成 |
| `semodule -i NAME.pp` | ポリシー適用（※ -X 300 未指定） |
| `semodule -lfull` | ポリシーモジュール一覧取得 |
| `semodule -X PRIORITY -r NAME` | ポリシーモジュール削除 |
| `restorecon -Rv PATH` | ラベル修復 |
| `semanage port -a -t TYPE -p PROTO PORT` | ポートコンテキスト追加 |
| `semanage fcontext -a -t TYPE PATTERN; restorecon -Rv PATH` | ファイルコンテキスト設定＋適用 |
| `semanage permissive -a DOMAIN` | ドメイン permissive 化 |
| `setsebool [-P] BOOL on` | Boolean 有効化 |
| `sepolicy booleans -b BOOL` | Boolean 説明文取得 |

### Remedy 自動診断

| Remedy 種別 | 診断トリガー（ヒューリスティック） |
|---|---|
| PortContext | TCP/UDP × name_bind/name_connect |
| FileContext | ファイル/ディレクトリ × 非汎用型 |
| Restorecon | unlabeled_t |
| Boolean | 既知サービス（httpd/ftpd/samba）の hardcode 4 エントリ |
| CustomPolicy | 上記いずれにも該当しない場合（フォールバック） |
