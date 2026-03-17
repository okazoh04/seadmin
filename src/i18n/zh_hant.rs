// ── 頁尾提示 ──────────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:移動  Enter:詳情  /:篩選  r:重新整理  l:日誌  q:退出";
pub const HINT_AVC_DETAIL:   &str = "A-F:選擇處置  Esc/←:返回  Enter:確認";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:捲動  Enter:套用  Esc:取消";
pub const HINT_AUTH:         &str = "Enter:執行  Esc:取消";

// ── 表格標題 ──────────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "時間";
pub const COL_PROCESS:  &str = "程序";
pub const COL_ACTION:   &str = "操作";
pub const COL_TARGET:   &str = "目標";
pub const COL_COUNT:    &str = "次數";
pub const COL_REMEDY:   &str = "修復方案";

// ── 狀態 / 訊息 ───────────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ 正在載入 AVC 日誌...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux 已停用。存取拒絕事件將不會被記錄。";
pub const NO_AVC:           &str = "沒有存取拒絕事件";
pub const OP_COMPLETE:      &str = "操作已完成";
pub const IGNORED:          &str = "已新增至忽略清單";
pub const FILTER_LABEL:     &str = "/篩選: ";

// ── 認證彈窗 ──────────────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 管理員認證";
pub const AUTH_CMD_LABEL:  &str = "  執行命令：";
pub const AUTH_PW_LABEL:   &str = "  密碼：";
pub const AUTH_CANCEL_BTN: &str = "[ 取消（Esc） ]";
pub const AUTH_EXEC_BTN:   &str = "[ 執行（Enter） ]";
pub const PW_WRONG:        &str = "密碼不正確";

// ── 詳情畫面區塊 ──────────────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " 原因分析 ";
pub const BLOCK_OPTIONS:  &str = " 處置選項 ";
pub const BLOCK_RAW_LOG:  &str = " 原始日誌（參考）";

// ── 原則審核 ──────────────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " 原則內容確認（Enter:套用  Esc:取消）";
pub const POLICY_APPLY_DESC:   &str = "將產生的原則模組套用至系統。";

// ── 處置選項（靜態） ──────────────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "還原預設檔案內容（修復標籤遺失問題）。";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "產生並套用自訂原則模組（audit2allow）";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "使用 audit2allow 自動產生原則。可在套用前進行審核。";
pub const OPT_PERMISSIVE_DESC:     &str = "暫時停用拒絕。會降低安全性，請僅用於調查目的。";
pub const OPT_IGNORE_LABEL:        &str = "不處理 / 新增至忽略清單";
pub const OPT_IGNORE_DESC:         &str = "將此項目新增至忽略清單（僅限工具內部）。";

// ── 原因分析（靜態） ──────────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " 非標準路徑需要新增 fcontext 規則。";
pub const ANALYSIS_RESTORECON_FIX:       &str = " 執行 restorecon 還原預設內容可能解決此問題。";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " 需要使用 audit2allow 產生自訂原則。";

// ── Remedy 顯示名稱 ───────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "新增連接埠";
pub const REMEDY_FILE_CONTEXT:  &str = "fcontext 變更";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "自訂原則";

// ── 格式化字串 ────────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" 存取拒絕清單  [今日]  未處理: {}條 / 共 {}條 ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("已載入 {} 個 AVC 項目", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("命令失敗: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  已鎖定（{}秒後解除）", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" 日誌  {} 條  ↑↓:捲動  l:關閉 ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("新增連接埠內容  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("為 {} 的 {} 連接埠分配 ssh_port_t 內容。", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("使用 restorecon 修復  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("變更 fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("新增將 {} 分配給此路徑的規則。套用後請同時執行 restorecon。", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("啟用 Boolean（暫時）  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("啟用 {}（重新開機後還原）。", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("啟用 Boolean（永久）  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("永久啟用 {}。", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("將網域設為 Permissive（僅用於調查）⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} 對 {} 的 {} 操作被拒絕。", process, target, perm)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" 連接埠 {} 在 SELinux 原則中未定義。", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} 在非標準連接埠執行需要新增連接埠內容。", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" 對 {} 的寫入被拒絕。", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" {} 的標籤可能已遺失。", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" 啟用 {} Boolean 可能解決此問題。", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" 網域 {} 的 {} 操作未被原則允許。", domain, perm)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}秒前", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}分前", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}小時前", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}天前", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "警告：區域設定可能不是 UTF-8 (LANG={})。\n\
         如果字元顯示不正確，請設定 LANG=zh_TW.UTF-8。",
        lang_val
    )
}
