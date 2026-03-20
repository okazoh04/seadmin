/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── 頁尾提示 ──────────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:移動  Enter:詳情  /:篩選  r:重新整理  m:模組  l:日誌  q:退出";
pub const HINT_AVC_DETAIL:   &str = "A-F:選擇處置  Esc/←:返回  Enter:確認";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:捲動  Enter:套用  Esc:取消";
pub const HINT_AUTH:         &str = "Enter:執行  Esc:取消";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:移動  d:刪除  Esc:返回";

// ── 表格標題 ──────────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "時間";
pub const COL_PROCESS:  &str = "程序";
pub const COL_ACTION:   &str = "操作";
pub const COL_TARGET:   &str = "目標";
pub const COL_COUNT:    &str = "次數";
pub const COL_REMEDY:       &str = "修復方案";
pub const COL_PRIORITY:     &str = "優先級";
pub const COL_MODULE_NAME:  &str = "模組名";

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
pub fn module_list_title(count: usize) -> String {
    format!(" 政策模組清單  {} 個 ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("刪除政策模組 '{}'。", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("模組 '{}' 已刪除。", name)
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
    format!("變更 fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("新增將 {} 分配給此路徑的規則，並自動執行 restorecon。", file_type)
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
pub const LABEL_FIRST_SEEN: &str = "首次發生";
pub const LABEL_LAST_SEEN:  &str = "最後發生";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "警告：區域設定可能不是 UTF-8 (LANG={})。\n\
         如果字元顯示不正確，請設定 LANG=zh_TW.UTF-8。",
        lang_val
    )
}

// ── 依賴檢查輸出 ──────────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] 部分功能所需的指令未找到:";
pub const WARN_MISSING_OPT_FTR: &str = "       使用上述指令的功能將無法工作。";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] 找不到必要指令，無法啟動 seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (套件: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
請安裝上述套件後重試。\n\
  例 (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  例 (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── 日誌輸出 ──────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin 已啟動 (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] 無法開啟日誌檔案: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC 已載入: {} 筆", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (非絕對路徑 — restorecon/fcontext 已隱藏)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC 載入失敗: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] 指令成功";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] 認證失敗 ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] 指令失敗:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux 模式: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow 已生成: {} 行, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} (輸入 {} 行日誌)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (快取認證)", cmd) }

// ── 指令錯誤 ──────────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "沒有讀取 audit.log 的權限。請將自己加入 adm 群組或設定 sudo。";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow 失敗: {}", stderr) }
