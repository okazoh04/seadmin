/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── คำแนะนำส่วนล่าง ───────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:เลื่อน  Enter:รายละเอียด  /:กรอง  r:โหลดใหม่  l:บันทึก  q:ออก";
pub const HINT_AVC_DETAIL:   &str = "A-F:เลือก  Esc/←:ย้อนกลับ  Enter:ยืนยัน";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:เลื่อน  Enter:นำไปใช้  Esc:ยกเลิก";
pub const HINT_AUTH:         &str = "Enter:ดำเนินการ  Esc:ยกเลิก";

// ── ส่วนหัวตาราง ─────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "เวลา";
pub const COL_PROCESS:  &str = "กระบวนการ";
pub const COL_ACTION:   &str = "การกระทำ";
pub const COL_TARGET:   &str = "เป้าหมาย";
pub const COL_COUNT:    &str = "จำนวน";
pub const COL_REMEDY:   &str = "วิธีแก้ไข";

// ── สถานะ / ข้อความ ──────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ กำลังโหลด AVC log...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux ถูกปิดใช้งาน การปฏิเสธการเข้าถึงจะไม่ถูกบันทึก";
pub const NO_AVC:           &str = "ไม่มีการปฏิเสธการเข้าถึง";
pub const OP_COMPLETE:      &str = "ดำเนินการเสร็จสิ้น";
pub const IGNORED:          &str = "เพิ่มลงในรายการละเว้นแล้ว";
pub const FILTER_LABEL:     &str = "/กรอง: ";

// ── หน้าต่างยืนยันตัวตน ──────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 การยืนยันตัวตนผู้ดูแลระบบ";
pub const AUTH_CMD_LABEL:  &str = "  คำสั่ง:";
pub const AUTH_PW_LABEL:   &str = "  รหัสผ่าน:";
pub const AUTH_CANCEL_BTN: &str = "[ ยกเลิก (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ ดำเนินการ (Enter) ]";
pub const PW_WRONG:        &str = "รหัสผ่านไม่ถูกต้อง";

// ── บล็อกหน้าจอรายละเอียด ────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " วิเคราะห์สาเหตุ ";
pub const BLOCK_OPTIONS:  &str = " ตัวเลือกการแก้ไข ";
pub const BLOCK_RAW_LOG:  &str = " บันทึกดิบ (อ้างอิง)";

// ── การตรวจสอบนโยบาย ─────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " ตรวจสอบนโยบาย (Enter:นำไปใช้  Esc:ยกเลิก)";
pub const POLICY_APPLY_DESC:   &str = "นำโมดูลนโยบายที่สร้างขึ้นไปใช้กับระบบ";

// ── ตัวเลือกการแก้ไข (คงที่) ─────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "คืนค่า file context เริ่มต้น (แก้ไข label ที่สูญหาย)";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "สร้างและนำโมดูลนโยบายที่กำหนดเองไปใช้ (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "สร้างนโยบายอัตโนมัติด้วย audit2allow ตรวจสอบก่อนนำไปใช้";
pub const OPT_PERMISSIVE_DESC:     &str = "ปิดใช้งานการปฏิเสธชั่วคราว ลดความปลอดภัย ใช้เพื่อการสืบสวนเท่านั้น";
pub const OPT_IGNORE_LABEL:        &str = "ไม่ทำอะไร / เพิ่มลงในรายการละเว้น";
pub const OPT_IGNORE_DESC:         &str = "เพิ่มรายการนี้ลงในรายการละเว้น (ภายในเครื่องมือเท่านั้น)";

// ── วิเคราะห์สาเหตุ (คงที่) ──────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " เส้นทางที่ไม่เป็นมาตรฐานต้องการเพิ่มกฎ fcontext";
pub const ANALYSIS_RESTORECON_FIX:       &str = " การรัน restorecon เพื่อคืนค่า context เริ่มต้นอาจแก้ปัญหาได้";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " ต้องสร้างนโยบายที่กำหนดเองด้วย audit2allow";

// ── ชื่อแสดง Remedy ──────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "เพิ่มพอร์ต";
pub const REMEDY_FILE_CONTEXT:  &str = "เปลี่ยน fcontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "นโยบายที่กำหนดเอง";

// ── สตริงรูปแบบ ──────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" การปฏิเสธการเข้าถึง  [วันนี้]  ยังไม่แก้ไข: {} / ทั้งหมด: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("โหลด {} รายการ AVC แล้ว", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("คำสั่งล้มเหลว: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  ถูกล็อก (เหลือ {} วินาที)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" บันทึก  {} รายการ  ↑↓:เลื่อน  l:ปิด ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("เพิ่ม port context  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("กำหนด context ssh_port_t ให้พอร์ต {} ของ {}", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("แก้ไขด้วย restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("เปลี่ยน fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("เพิ่มกฎเพื่อกำหนด {} ให้เส้นทางนี้ รัน restorecon หลังจากนำไปใช้", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("เปิดใช้ Boolean (ชั่วคราว)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("เปิดใช้ {} (จะถูกรีเซ็ตเมื่อรีบูต)", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("เปิดใช้ Boolean (ถาวร)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("เปิดใช้ {} แบบถาวร", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("ตั้ง domain เป็น Permissive (เพื่อการสืบสวนเท่านั้น) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} ถูกปฏิเสธ {} บน {}", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" พอร์ต {} ไม่ได้กำหนดไว้ในนโยบาย SELinux", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} ต้องการเพิ่ม port context เพื่อทำงานบนพอร์ตที่ไม่เป็นมาตรฐาน", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" การเขียนไปยัง {} ถูกปฏิเสธ", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" label บน {} อาจถูกลบออก", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" การเปิดใช้ Boolean {} อาจแก้ปัญหาได้", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" การดำเนินการ {} จาก domain {} ไม่ได้รับอนุญาตตามนโยบาย", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{} วินาทีที่แล้ว", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{} นาทีที่แล้ว", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{} ชั่วโมงที่แล้ว", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{} วันที่แล้ว", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "คำเตือน: locale อาจไม่ใช่ UTF-8 (LANG={})\n\
         ตั้งค่า LANG=th_TH.UTF-8 หากอักขระแสดงผลไม่ถูกต้อง",
        lang_val
    )
}
