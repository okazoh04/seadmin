// ── Footer hints ─────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Move  Enter:Detail  /:Filter  r:Reload  l:Log  q:Quit";
pub const HINT_AVC_DETAIL:   &str = "A-F:Select  Esc/←:Back  Enter:Confirm";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Scroll  Enter:Apply  Esc:Cancel";
pub const HINT_AUTH:         &str = "Enter:Execute  Esc:Cancel";

// ── Table headers ─────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "When";
pub const COL_PROCESS:  &str = "Process";
pub const COL_ACTION:   &str = "Action";
pub const COL_TARGET:   &str = "Target";
pub const COL_COUNT:    &str = "Count";
pub const COL_REMEDY:   &str = "Remedy";

// ── Status / messages ─────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Loading AVC log...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux is disabled. Access denials will not be recorded.";
pub const NO_AVC:           &str = "No access denials";
pub const OP_COMPLETE:      &str = "Operation completed";
pub const IGNORED:          &str = "Added to ignore list";
pub const FILTER_LABEL:     &str = "/Filter: ";

// ── Auth popup ────────────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Administrator Authentication";
pub const AUTH_CMD_LABEL:  &str = "  Command:";
pub const AUTH_PW_LABEL:   &str = "  Password:";
pub const AUTH_CANCEL_BTN: &str = "[ Cancel (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Execute (Enter) ]";
pub const PW_WRONG:        &str = "Incorrect password";

// ── Detail screen blocks ──────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Analysis ";
pub const BLOCK_OPTIONS:  &str = " Remediation Options ";
pub const BLOCK_RAW_LOG:  &str = " Raw Log (reference)";

// ── Policy review ─────────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Policy Review (Enter:Apply  Esc:Cancel)";
pub const POLICY_APPLY_DESC:   &str = "Apply the generated policy module to the system.";

// ── Remediation options (static) ──────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Restore the default file context (repair stripped labels).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Generate and apply custom policy module (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Auto-generate a policy with audit2allow. Review before applying.";
pub const OPT_PERMISSIVE_DESC:     &str = "Temporarily disable denials. Reduces security; use only for investigation.";
pub const OPT_IGNORE_LABEL:        &str = "Do nothing / Add to ignore list";
pub const OPT_IGNORE_DESC:         &str = "Add this entry to the ignore list (tool-local only).";

// ── Analysis (static) ─────────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Non-standard path requires adding an fcontext rule.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Running restorecon to restore the default context may resolve this.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " A custom policy needs to be generated with audit2allow.";

// ── Remedy display names ──────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Port Context";
pub const REMEDY_FILE_CONTEXT:  &str = "File Context";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Custom Policy";

// ── Format strings ────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Access Denials  [Today]  Unresolved: {} / Total: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("Loaded {} AVC entries", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Command failed: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Locked out ({} seconds remaining)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Log  {} entries  ↑↓:Scroll  l:Close ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Add port context  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Assign ssh_port_t context to {} port {}.", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Repair with restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Change fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Add a rule to assign {} to this path. Run restorecon after applying.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Enable Boolean (temporary)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Enable {} (reverts after reboot).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Enable Boolean (persistent)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Persistently enable {}.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Set domain to Permissive (investigation only) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} was denied {} on {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Port {} is not defined in the SELinux policy.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} needs a port context added to operate on a non-standard port.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Write access to {} was denied.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" The label on {} may have been stripped.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Enabling the {} Boolean may resolve this.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" {} operation from domain {} is not allowed by policy.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}s ago", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}m ago", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}h ago", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}d ago", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Warning: Locale may not be UTF-8 (LANG={}).\n\
         Set LANG=en_US.UTF-8 if characters display incorrectly.",
        lang_val
    )
}
