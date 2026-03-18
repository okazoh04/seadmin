/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── تلميحات التذييل ───────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:تنقل  Enter:تفاصيل  /:تصفية  r:تحديث  l:سجل  q:خروج";
pub const HINT_AVC_DETAIL:   &str = "A-F:اختيار  Esc/←:رجوع  Enter:تأكيد";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:تمرير  Enter:تطبيق  Esc:إلغاء";
pub const HINT_AUTH:         &str = "Enter:تنفيذ  Esc:إلغاء";

// ── رؤوس الجدول ──────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "الوقت";
pub const COL_PROCESS:  &str = "العملية";
pub const COL_ACTION:   &str = "الإجراء";
pub const COL_TARGET:   &str = "الهدف";
pub const COL_COUNT:    &str = "العدد";
pub const COL_REMEDY:   &str = "الحل";

// ── الحالة / الرسائل ──────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ جارٍ تحميل سجل AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux معطّل. لن يتم تسجيل رفض الوصول.";
pub const NO_AVC:           &str = "لا يوجد رفض وصول";
pub const OP_COMPLETE:      &str = "اكتملت العملية";
pub const IGNORED:          &str = "تمت الإضافة إلى قائمة التجاهل";
pub const FILTER_LABEL:     &str = "/تصفية: ";

// ── نافذة المصادقة ────────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 مصادقة المسؤول";
pub const AUTH_CMD_LABEL:  &str = "  الأمر:";
pub const AUTH_PW_LABEL:   &str = "  كلمة المرور:";
pub const AUTH_CANCEL_BTN: &str = "[ إلغاء (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ تنفيذ (Enter) ]";
pub const PW_WRONG:        &str = "كلمة مرور غير صحيحة";

// ── كتل شاشة التفاصيل ────────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " تحليل السبب ";
pub const BLOCK_OPTIONS:  &str = " خيارات الحل ";
pub const BLOCK_RAW_LOG:  &str = " السجل الخام (للمراجعة)";

// ── مراجعة السياسة ───────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " مراجعة السياسة (Enter:تطبيق  Esc:إلغاء)";
pub const POLICY_APPLY_DESC:   &str = "تطبيق وحدة السياسة المولّدة على النظام.";

// ── خيارات الحل (ثابتة) ──────────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "استعادة سياق الملف الافتراضي (إصلاح التسميات المفقودة).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "إنشاء وتطبيق وحدة سياسة مخصصة (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "إنشاء سياسة تلقائياً باستخدام audit2allow. راجع قبل التطبيق.";
pub const OPT_PERMISSIVE_DESC:     &str = "تعطيل الرفض مؤقتاً. يقلل الأمان؛ استخدم فقط للتحقيق.";
pub const OPT_IGNORE_LABEL:        &str = "لا شيء / إضافة إلى قائمة التجاهل";
pub const OPT_IGNORE_DESC:         &str = "إضافة هذا الإدخال إلى قائمة التجاهل (داخل الأداة فقط).";

// ── تحليل السبب (ثابت) ───────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " المسار غير القياسي يتطلب إضافة قاعدة fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " تشغيل restorecon لاستعادة السياق الافتراضي قد يحل المشكلة.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " يجب إنشاء سياسة مخصصة باستخدام audit2allow.";

// ── أسماء عرض Remedy ─────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "سياق المنفذ";
pub const REMEDY_FILE_CONTEXT:  &str = "سياق الملف";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "سياسة مخصصة";

// ── سلاسل التنسيق ────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" رفض الوصول  [اليوم]  غير محلول: {} / الإجمالي: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("تم تحميل {} إدخال AVC", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("فشل الأمر: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  مقفل ({} ثانية متبقية)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" السجل  {} إدخال  ↑↓:تمرير  l:إغلاق ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("إضافة سياق منفذ  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("تعيين سياق ssh_port_t للمنفذ {} من {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("إصلاح باستخدام restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("تغيير fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("إضافة قاعدة لتعيين {} لهذا المسار. شغّل restorecon بعد التطبيق.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("تفعيل Boolean (مؤقت)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("تفعيل {} (يُعاد ضبطه عند إعادة التشغيل).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("تفعيل Boolean (دائم)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("تفعيل {} بشكل دائم.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("ضبط النطاق على Permissive (للتحقيق فقط) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" تم رفض {} لـ{} على {}.", perm, process, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" المنفذ {} غير معرّف في سياسة SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} يحتاج سياق منفذ للعمل على منفذ غير قياسي.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" تم رفض الوصول للكتابة إلى {}.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" قد تكون التسمية على {} قد أُزيلت.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" تفعيل Boolean {} قد يحل المشكلة.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" العملية {} من النطاق {} غير مسموح بها بموجب السياسة.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("منذ {}ث", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("منذ {}د", n) }
pub fn elapsed_hours(n: u64) -> String { format!("منذ {}س", n) }
pub fn elapsed_days(n: u64)  -> String { format!("منذ {}ي", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "تحذير: قد لا يكون الإعداد المحلي UTF-8 (LANG={}).\n\
         اضبط LANG=ar_SA.UTF-8 إذا لم تظهر الأحرف بشكل صحيح.",
        lang_val
    )
}

// ── فحص التبعيات ──────────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] بعض الأوامر المطلوبة غير موجودة:";
pub const WARN_MISSING_OPT_FTR: &str = "       الميزات التي تستخدم الأوامر أعلاه لن تعمل.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] الأوامر المطلوبة غير موجودة. لا يمكن تشغيل seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (الحزمة: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
يرجى تثبيت الحزم أعلاه والمحاولة مرة أخرى.\n\
  مثال (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  مثال (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── إخراج السجل ───────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] بدأ تشغيل seadmin (السجل: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] تعذّر فتح ملف السجل: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] تم تحميل AVC: {} إدخال", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (لا مسار مطلق — restorecon/fcontext مخفي)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] فشل تحميل AVC: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] نجح الأمر";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] فشل المصادقة ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] فشل الأمر:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] وضع SELinux: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] تم إنشاء audit2allow: {} سطر، pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} سطر سجل كمدخل)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (مصادقة مخزنة مؤقتاً)", cmd) }

// ── أخطاء الأوامر ─────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "لا توجد صلاحية لقراءة audit.log. أضف نفسك إلى مجموعة adm أو قم بتكوين sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("فشل audit2allow: {}", stderr) }
