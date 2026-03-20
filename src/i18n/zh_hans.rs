/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── 页脚提示 ──────────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:移动  Enter:详情  /:过滤  r:刷新  m:模块  l:日志  q:退出";
pub const HINT_AVC_DETAIL:   &str = "A-F:选择处置  Esc/←:返回  Enter:确认";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:滚动  Enter:应用  Esc:取消";
pub const HINT_AUTH:         &str = "Enter:执行  Esc:取消";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:移动  d:删除  Esc:返回";

// ── 表格标题 ──────────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "时间";
pub const COL_PROCESS:  &str = "进程";
pub const COL_ACTION:   &str = "操作";
pub const COL_TARGET:   &str = "目标";
pub const COL_COUNT:    &str = "次数";
pub const COL_REMEDY:       &str = "修复方案";
pub const COL_PRIORITY:     &str = "优先级";
pub const COL_MODULE_NAME:  &str = "模块名";

// ── 状态 / 消息 ───────────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ 正在加载 AVC 日志...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux 已禁用。访问拒绝事件将不会被记录。";
pub const NO_AVC:           &str = "没有访问拒绝事件";
pub const OP_COMPLETE:      &str = "操作已完成";
pub const IGNORED:          &str = "已添加到忽略列表";
pub const FILTER_LABEL:     &str = "/过滤: ";

// ── 认证弹窗 ──────────────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 管理员认证";
pub const AUTH_CMD_LABEL:  &str = "  执行命令：";
pub const AUTH_PW_LABEL:   &str = "  密码：";
pub const AUTH_CANCEL_BTN: &str = "[ 取消（Esc） ]";
pub const AUTH_EXEC_BTN:   &str = "[ 执行（Enter） ]";
pub const PW_WRONG:        &str = "密码不正确";

// ── 详情画面区块 ──────────────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " 原因分析 ";
pub const BLOCK_OPTIONS:  &str = " 处置选项 ";
pub const BLOCK_RAW_LOG:  &str = " 原始日志（参考）";

// ── 策略审核 ──────────────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " 策略内容确认（Enter:应用  Esc:取消）";
pub const POLICY_APPLY_DESC:   &str = "将生成的策略模块应用到系统。";

// ── 处置选项（静态） ──────────────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "恢复默认文件上下文（修复标签丢失问题）。";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "生成并应用自定义策略模块（audit2allow）";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "使用 audit2allow 自动生成策略。可在应用前进行审核。";
pub const OPT_PERMISSIVE_DESC:     &str = "临时禁用拒绝。会降低安全性，请仅用于调查目的。";
pub const OPT_IGNORE_LABEL:        &str = "不处理 / 添加到忽略列表";
pub const OPT_IGNORE_DESC:         &str = "将此条目添加到忽略列表（仅限工具内部）。";

// ── 原因分析（静态） ──────────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " 非标准路径需要添加 fcontext 规则。";
pub const ANALYSIS_RESTORECON_FIX:       &str = " 运行 restorecon 恢复默认上下文可能解决此问题。";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " 需要使用 audit2allow 生成自定义策略。";

// ── Remedy 显示名 ─────────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "添加端口";
pub const REMEDY_FILE_CONTEXT:  &str = "fcontext更改";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "自定义策略";

// ── 格式化字符串 ──────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" 访问拒绝列表  [今日]  未处理: {}条 / 共 {}条 ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" 策略模块列表  {} 个 ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("删除策略模块 '{}'。", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("模块 '{}' 已删除。", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("已加载 {} 个 AVC 条目", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("命令失败: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  已锁定（{}秒后解除）", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" 日志  {} 条  ↑↓:滚动  l:关闭 ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("添加端口上下文  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("为 {} 的 {} 端口分配 ssh_port_t 上下文。", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("使用 restorecon 修复  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("更改 fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("添加将 {} 分配给此路径的规则，并自动运行 restorecon。", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("启用 Boolean（临时）  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("启用 {}（重启后恢复）。", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("启用 Boolean（持久）  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("持久启用 {}。", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("将域设为 Permissive（仅用于调查）⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} 对 {} 的 {} 操作被拒绝。", process, target, perm)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" 端口 {} 在 SELinux 策略中未定义。", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} 在非标准端口运行需要添加端口上下文。", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" 对 {} 的写入被拒绝。", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" {} 的标签可能已丢失。", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" 启用 {} Boolean 可能解决此问题。", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" 域 {} 的 {} 操作未被策略允许。", domain, perm)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}秒前", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}分前", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}小时前", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}天前", n) }
pub const LABEL_FIRST_SEEN: &str = "首次发生";
pub const LABEL_LAST_SEEN:  &str = "最后发生";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "警告: 区域设置可能不是 UTF-8 (LANG={})。\n\
         如果字符显示不正确，请设置 LANG=zh_CN.UTF-8。",
        lang_val
    )
}

// ── 依赖检查输出 ──────────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] 部分功能所需的命令未找到:";
pub const WARN_MISSING_OPT_FTR: &str = "       使用上述命令的功能将无法工作。";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] 找不到必要命令，无法启动 seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (软件包: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
请安装上述软件包后重试。\n\
  例 (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  例 (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── 日志输出 ──────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin 已启动 (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] 无法打开日志文件: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC 已加载: {} 条", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (非绝对路径 — restorecon/fcontext 已隐藏)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC 加载失败: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] 命令成功";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] 认证失败 ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] 命令失败:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux 模式: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow 已生成: {} 行, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} (输入 {} 行日志)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (缓存认证)", cmd) }

// ── 命令错误 ──────────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "没有读取 audit.log 的权限。请将自己添加到 adm 组或配置 sudo。";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow 失败: {}", stderr) }
