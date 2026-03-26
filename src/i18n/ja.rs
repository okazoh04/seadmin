/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── フッターヒント ────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:移動  Enter:詳細  /:フィルタ  r:更新  m:モジュール  l:ログ  q:終了";
pub const HINT_AVC_DETAIL:   &str = "A-F:対処選択  Esc/←:戻る  Enter:確認へ";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:スクロール  Enter:適用  Esc:キャンセル";
pub const HINT_AUTH:         &str = "Enter:実行  Esc:キャンセル";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:移動  d:削除  Esc:戻る";

// ── テーブルヘッダー ─────────────────────────────────────────────────────────
pub const COL_OCCURRED:     &str = "発生";
pub const COL_PROCESS:      &str = "プロセス";
pub const COL_ACTION:       &str = "操作";
pub const COL_TARGET:       &str = "対象";
pub const COL_COUNT:        &str = "件数";
pub const COL_REMEDY:       &str = "解決策候補";
pub const COL_PRIORITY:     &str = "優先度";
pub const COL_MODULE_NAME:  &str = "モジュール名";

// ── ステータス・メッセージ ────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ AVC ログを読み込み中...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux が無効です。アクセス拒否は記録されません。";
pub const NO_AVC:           &str = "アクセス拒否はありません";
pub const OP_COMPLETE:      &str = "操作が完了しました";
pub const IGNORED:          &str = "無視リストに追加しました";
pub const FILTER_LABEL:     &str = "/フィルタ: ";

// ── 認証ポップアップ ─────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 管理者認証";
pub const AUTH_CMD_LABEL:  &str = "  実行コマンド：";
pub const AUTH_PW_LABEL:   &str = "  パスワード：";
pub const AUTH_CANCEL_BTN: &str = "[ キャンセル（Esc） ]";
pub const AUTH_EXEC_BTN:   &str = "[ 実行（Enter） ]";
pub const PW_WRONG:        &str = "パスワードが正しくありません";

// ── 詳細画面ブロック ─────────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " 原因分析 ";
pub const BLOCK_OPTIONS:  &str = " 対処オプション ";
pub const BLOCK_RAW_LOG:  &str = " 生ログ（参照用）";

// ── ポリシーレビュー ─────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " ポリシー内容確認（Enter:適用  Esc:キャンセル）";
pub const POLICY_APPLY_DESC:   &str = "生成したポリシーモジュールをシステムに適用します。";

// ── 対処オプション（静的） ────────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:   &str = "まず試してください。ラベルが剥がれた場合はこれで解決します。";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "カスタムポリシーモジュールを生成・適用（audit2allow）";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "audit2allow でポリシーを自動生成します。パスが判明している場合は先に P キーを試してください。";
pub const OPT_PERMISSIVE_DESC:   &str = "⚠ ドメイン全体の拒否を無効化します。セキュリティが大幅に低下します。調査目的の一時措置のみ。";
pub const OPT_IGNORE_LABEL:      &str = "何もしない / 無視リストに追加";
pub const OPT_IGNORE_DESC:       &str = "このエントリを無視リストに追加します（ツール内のみ）。";

// ── 原因分析（静的） ─────────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " 非標準パスのため fcontext ルールの追加が必要です。";
pub const ANALYSIS_RESTORECON_FIX:       &str = " restorecon でデフォルトコンテキストに戻すことで解決できます。";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " audit2allow でカスタムポリシーを生成する必要があります。";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " ※ パスが不明です。P キーでパスを指定すると最善策が表示されます。";
pub const PATH_INPUT_TITLE:  &str = " ディレクトリパスを入力";
pub const PATH_INPUT_PROMPT: &str = " 絶対パスを入力してください（例: /var/log/myapp）";
pub const PATH_INPUT_HINT:   &str = " Enter: 確定  Esc: キャンセル";
pub const OPT_PATH_INPUT_LABEL: &str = "絶対パスを入力して restorecon/fcontext を有効化";
pub const OPT_PATH_INPUT_DESC:  &str = "パスが不明なため A・B の修正手順を表示できません。絶対パスを入力すると、ラベル修正（restorecon / semanage fcontext）の対処方法が表示されます。";
// ── Remedy 表示名 ────────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "ポート追加";
pub const REMEDY_FILE_CONTEXT:  &str = "fcontext変更";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "カスタムポリシー";

// ── フォーマット文字列 ────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" アクセス拒否一覧  [本日]  未対処: {}件 / 全 {}件 ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" ポリシーモジュール一覧  {} 件 ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("ポリシーモジュール '{}' を削除します。", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("モジュール '{}' を削除しました。", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} 件の AVC を取得しました", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("コマンド失敗: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  ロックアウト中（{}秒後に解除）", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" ログ  {} 件  ↑↓:スクロール  Esc:閉じる ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("ポートコンテキストを追加  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("{} の {} ポートに ssh_port_t コンテキストを付与します。", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("restorecon で修復  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("fcontext 変更 + restorecon 実行  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("このパスに {} を付与するルールを追加し、restorecon を自動実行します。", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Boolean を有効化（一時）  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("{} を有効にします（再起動で元に戻ります）。", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Boolean を有効化（永続）  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("{} を永続的に有効にします。", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("このドメインを Permissive に設定（調査用）⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} が {} への {} を拒否されました。", process, target, perm)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" ポート {} は SELinux ポリシー上で未定義です。", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} を非標準ポートで動作させるにはポートコンテキストの追加が必要です。", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" {} への書き込みが拒否されました。", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" {} のラベルが剥がれている可能性があります。", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" ls -dZ {} でラベルを確認してください。不正なら restorecon を先に試してください。", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" {} Boolean を有効にすることで解決できる可能性があります。", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" ドメイン {} からの {} 操作がポリシーで許可されていません。", domain, perm)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}秒前", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}分前", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}時間前", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}日前", n) }
pub const LABEL_FIRST_SEEN: &str = "初回発生";
pub const LABEL_LAST_SEEN:  &str = "最終発生";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "警告: ロケールが UTF-8 ではない可能性があります（LANG={}）。\n\
         日本語が正しく表示されない場合は LANG=ja_JP.UTF-8 を設定してください。",
        lang_val
    )
}

// ── check_deps 出力 ───────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] 一部の機能に必要なコマンドが見つかりません:";
pub const WARN_MISSING_OPT_FTR: &str = "       上記コマンドを使う機能は動作しません。";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] 必須コマンドが見つかりません。seadmin を起動できません:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (パッケージ: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
上記パッケージをインストールしてから再実行してください。\n\
  例 (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  例 (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── ログ出力 ──────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String {
    format!("[INFO] seadmin 起動 (log: {})", path)
}
pub fn log_file_open_error(err: &str) -> String {
    format!("[WARN] ログファイルを開けませんでした: {}", err)
}
pub fn log_avc_loaded_n(count: usize) -> String {
    format!("[INFO] AVC ロード: {} 件", count)
}
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (絶対パス不明 → restorecon/fcontext 非表示)", target)
}
pub fn log_avc_load_error(err: &str) -> String {
    format!("[ERR] AVC ロード失敗: {}", err)
}
pub const LOG_CMD_OK: &str = "[OK] コマンド成功";
pub fn log_auth_failed(n: u32) -> String {
    format!("[ERR] 認証失敗 ({}/3)", n)
}
pub fn log_cmd_failed_msg(stderr: &str) -> String {
    format!("[ERR] コマンド失敗:\n{}", stderr)
}
pub fn log_selinux_mode(mode: &str) -> String {
    format!("[INFO] SELinux モード: {}", mode)
}
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow 生成完了: {} 行, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} 行のログを入力)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String {
    format!("[CMD] sudo {} (キャッシュ認証)", cmd)
}

// ── コマンドエラー ────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "audit.log を読む権限がありません。adm グループへの追加か sudo 設定を確認してください。";
pub fn err_audit2allow_failed(stderr: &str) -> String {
    format!("audit2allow 失敗: {}", stderr)
}
