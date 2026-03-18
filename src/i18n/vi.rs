/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Gợi ý chân trang ──────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Di chuyển  Enter:Chi tiết  /:Lọc  r:Tải lại  l:Nhật ký  q:Thoát";
pub const HINT_AVC_DETAIL:   &str = "A-F:Chọn  Esc/←:Quay lại  Enter:Xác nhận";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Cuộn  Enter:Áp dụng  Esc:Hủy";
pub const HINT_AUTH:         &str = "Enter:Thực thi  Esc:Hủy";

// ── Tiêu đề bảng ──────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Thời gian";
pub const COL_PROCESS:  &str = "Tiến trình";
pub const COL_ACTION:   &str = "Hành động";
pub const COL_TARGET:   &str = "Mục tiêu";
pub const COL_COUNT:    &str = "Số lần";
pub const COL_REMEDY:   &str = "Giải pháp";

// ── Trạng thái / thông báo ────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Đang tải nhật ký AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux đã bị tắt. Các lần từ chối truy cập sẽ không được ghi lại.";
pub const NO_AVC:           &str = "Không có lần từ chối truy cập";
pub const OP_COMPLETE:      &str = "Thao tác hoàn tất";
pub const IGNORED:          &str = "Đã thêm vào danh sách bỏ qua";
pub const FILTER_LABEL:     &str = "/Lọc: ";

// ── Cửa sổ xác thực ──────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Xác thực quản trị viên";
pub const AUTH_CMD_LABEL:  &str = "  Lệnh:";
pub const AUTH_PW_LABEL:   &str = "  Mật khẩu:";
pub const AUTH_CANCEL_BTN: &str = "[ Hủy (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Thực thi (Enter) ]";
pub const PW_WRONG:        &str = "Mật khẩu không đúng";

// ── Khối màn hình chi tiết ────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Phân tích nguyên nhân ";
pub const BLOCK_OPTIONS:  &str = " Tùy chọn giải pháp ";
pub const BLOCK_RAW_LOG:  &str = " Nhật ký thô (tham khảo)";

// ── Xem xét chính sách ────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Xem xét chính sách (Enter:Áp dụng  Esc:Hủy)";
pub const POLICY_APPLY_DESC:   &str = "Áp dụng mô-đun chính sách đã tạo vào hệ thống.";

// ── Tùy chọn giải pháp (tĩnh) ────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Khôi phục ngữ cảnh tệp mặc định (sửa nhãn bị mất).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Tạo và áp dụng mô-đun chính sách tùy chỉnh (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Tự động tạo chính sách với audit2allow. Xem xét trước khi áp dụng.";
pub const OPT_PERMISSIVE_DESC:     &str = "Tạm thời vô hiệu hóa từ chối. Giảm bảo mật; chỉ dùng để điều tra.";
pub const OPT_IGNORE_LABEL:        &str = "Không làm gì / Thêm vào danh sách bỏ qua";
pub const OPT_IGNORE_DESC:         &str = "Thêm mục này vào danh sách bỏ qua (chỉ trong công cụ).";

// ── Phân tích nguyên nhân (tĩnh) ─────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Đường dẫn không chuẩn yêu cầu thêm quy tắc fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Chạy restorecon để khôi phục ngữ cảnh mặc định có thể giải quyết vấn đề.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Cần tạo chính sách tùy chỉnh bằng audit2allow.";

// ── Tên hiển thị Remedy ───────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Thêm cổng";
pub const REMEDY_FILE_CONTEXT:  &str = "Thay đổi fcontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Chính sách tùy chỉnh";

// ── Chuỗi định dạng ───────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Từ chối truy cập  [Hôm nay]  Chưa giải quyết: {} / Tổng: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("Đã tải {} mục AVC", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Lệnh thất bại: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Bị khóa (còn {} giây)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Nhật ký  {} mục  ↑↓:Cuộn  l:Đóng ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Thêm ngữ cảnh cổng  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Gán ngữ cảnh ssh_port_t cho cổng {} của {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Sửa với restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Thay đổi fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Thêm quy tắc để gán {} cho đường dẫn này. Chạy restorecon sau khi áp dụng.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Bật Boolean (tạm thời)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Bật {} (sẽ được đặt lại khi khởi động lại).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Bật Boolean (vĩnh viễn)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Bật {} vĩnh viễn.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Đặt miền thành Permissive (chỉ để điều tra) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} bị từ chối {} trên {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Cổng {} không được định nghĩa trong chính sách SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} cần ngữ cảnh cổng để hoạt động trên cổng không chuẩn.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Quyền ghi vào {} bị từ chối.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Nhãn trên {} có thể đã bị xóa.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Bật Boolean {} có thể giải quyết vấn đề này.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" Thao tác {} từ miền {} không được phép theo chính sách.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{} giây trước", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{} phút trước", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{} giờ trước", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{} ngày trước", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Cảnh báo: Locale có thể không phải UTF-8 (LANG={}).\n\
         Đặt LANG=vi_VN.UTF-8 nếu ký tự hiển thị không đúng.",
        lang_val
    )
}

// ── Kiểm tra phụ thuộc ────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Một số lệnh cần thiết không được tìm thấy:";
pub const WARN_MISSING_OPT_FTR: &str = "       Các tính năng sử dụng các lệnh trên sẽ không hoạt động.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Không tìm thấy các lệnh bắt buộc. Không thể khởi động seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (gói: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Hãy cài đặt các gói trên và thử lại.\n\
  vd. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  vd. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Đầu ra nhật ký ────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin đã khởi động (nhật ký: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Không thể mở tệp nhật ký: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] Đã tải AVC: {} mục", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (không có đường dẫn tuyệt đối — ẩn restorecon/fcontext)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] Tải AVC thất bại: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Lệnh thành công";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Xác thực thất bại ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Lệnh thất bại:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] Chế độ SELinux: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow đã tạo: {} dòng, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} dòng nhật ký làm đầu vào)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (xác thực đã lưu cache)", cmd) }

// ── Lỗi lệnh ──────────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Không có quyền đọc audit.log. Thêm bản thân vào nhóm adm hoặc cấu hình sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow thất bại: {}", stderr) }
