/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Төменгі бөліктегі кеңестер ────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Жылжыту  Enter:Толық  /:Сүзгі  r:Жаңарту  m:Модульдер  l:Журнал  q:Шығу";
pub const HINT_AVC_DETAIL:   &str = "A-F:Іс-әрекет  Esc/←:Артқа  Enter:Растау";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Айналдыру  Enter:Қолдану  Esc:Болдырмау";
pub const HINT_AUTH:         &str = "Enter:Орындау  Esc:Болдырмау";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Жылжыту  d:Жою  a:Барлық/Жеке  Esc:Артқа";
pub const MODULE_LIST_AUTH_DESC: &str = "Модуль тізімін алу үшін әкімші құқықтары қажет. Аутентификациядан кейін автоматты түрде жүктеледі.";
pub const HINT_MODULE_DETAIL:    &str = "↑↓/jk:Айналдыру  Esc:Артқа";
pub const MODULE_DETAIL_AUTH_DESC:  &str = "Модуль мәліметтерін алу үшін әкімші құқықтары қажет. Аутентификациядан кейін автоматты жүктеледі.";
pub const MODULE_DETAIL_TITLE:      &str = " Модуль мәліметтері ";
pub const MODULE_DETAIL_CIL_TITLE:  &str = " CIL ережелері ";

// ── Кесте тақырыптары ─────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Уақыт";
pub const COL_PROCESS:  &str = "Процесс";
pub const COL_ACTION:   &str = "Іс-әрекет";
pub const COL_TARGET:   &str = "Нысан";
pub const COL_COUNT:    &str = "Саны";
pub const COL_REMEDY:       &str = "Шешім";
pub const COL_PRIORITY:     &str = "Басымдық";
pub const COL_MODULE_NAME:  &str = "Модуль атауы";

// ── Күй / хабарлар ────────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ AVC журналы жүктелуде...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux өшірілген. Кіруден бас тарту жазылмайды.";
pub const NO_AVC:           &str = "Кіруден бас тарту жоқ";
pub const OP_COMPLETE:      &str = "Операция аяқталды";
pub const IGNORED:          &str = "Еленбеу тізіміне қосылды";
pub const FILTER_LABEL:     &str = "/Сүзгі: ";

// ── Аутентификация терезесі ───────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Әкімші аутентификациясы";
pub const AUTH_CMD_LABEL:  &str = "  Пәрмен:";
pub const AUTH_PW_LABEL:   &str = "  Құпия сөз:";
pub const AUTH_CANCEL_BTN: &str = "[ Болдырмау (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Орындау (Enter) ]";
pub const PW_WRONG:        &str = "Құпия сөз дұрыс емес";

// ── Толық экран блоктары ──────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Себеп талдауы ";
pub const BLOCK_OPTIONS:  &str = " Шешім нұсқалары ";
pub const BLOCK_RAW_LOG:  &str = " Бастапқы журнал (анықтамалық)";

// ── Саясатты тексеру ──────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Саясатты тексеру (Enter:Қолдану  Esc:Болдырмау)";
pub const POLICY_APPLY_DESC:   &str = "Жасалған саясат модулін жүйеге қолдану.";

// ── Шешім нұсқалары (статикалық) ─────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Алдымен осыны сынап көріңіз. Файлдың әдепкі контекстін қалпына келтіреді.";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Теңшелген саясат модулін жасау және қолдану (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "audit2allow арқылы саясатты автоматты жасау. Жол белгілі болса, алдымен P пернесін басыңыз.";
pub const OPT_PERMISSIVE_DESC:     &str = "⚠ Домен үшін барлық бас тартуларды өшіреді. Қауіпсіздікті күрт төмендетеді. Тек тексеру үшін.";
pub const OPT_IGNORE_LABEL:        &str = "Ешнәрсе жасамау / Еленбеу тізіміне қосу";
pub const OPT_IGNORE_DESC:         &str = "Бұл жазбаны еленбеу тізіміне қосу (тек құрал ішінде).";

// ── Себеп талдауы (статикалық) ────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Стандартты емес жол fcontext ережесін қосуды қажет етеді.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " restorecon арқылы әдепкі контекстті қалпына келтіру мәселені шешуі мүмкін.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " audit2allow арқылы теңшелген саясат жасау қажет.";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " * Жол белгісіз. P пернесін басып каталог жолын енгізіңіз.";
pub const PATH_INPUT_TITLE:  &str = " Каталог жолын енгізіңіз";
pub const PATH_INPUT_PROMPT: &str = " Толық жолды енгізіңіз (мысалы: /var/log/myapp)";
pub const PATH_INPUT_HINT:   &str = " Enter: Растау  Esc: Болдырмау";
pub const OPT_PATH_INPUT_LABEL: &str = "restorecon/fcontext қосу үшін абсолютті жол енгізіңіз";
pub const OPT_PATH_INPUT_DESC:  &str = "Жол белгісіз — A/B нұсқалары қолжетімсіз. Абсолютті жолды енгізсеңіз, таңба түзету қадамдары көрсетіледі.";

// ── Remedy атаулары ───────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Порт қосу";
pub const REMEDY_FILE_CONTEXT:  &str = "fcontext өзгерту";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Теңшелген саясат";

// ── Пішімдеу жолдары ──────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Кіруден бас тарту  [Бүгін]  Өңделмеген: {} / Барлығы: {} ", unresolved, total)
}
pub fn module_list_title(custom: usize, total: usize, show_all: bool) -> String {
    if show_all {
        format!(" Саясат модульдері  {} дана ", total)
    } else {
        format!(" Жеке модульдер  {} / барлығы {} ", custom, total)
    }
}
pub fn module_delete_desc(name: &str) -> String {
    format!("'{}' саясат модулін жою.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("'{}' модулі жойылды.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} AVC жазбасы жүктелді", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Пәрмен сәтсіз: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Бұғатталған ({} сек қалды)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Журнал  {} жазба  ↑↓:Айналдыру  Esc:Жабу ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Порт контекстін қосу  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("{} порты {}  ssh_port_t контекстін тағайындау.", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("restorecon арқылы қалпына келтіру  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("fcontext өзгерту + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Осы жолға {} тағайындайтын ереже қосып, restorecon автоматты іске қосылады.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Boolean қосу (уақытша)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("{} қосу (қайта жүктеуден кейін қалпына келеді).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Boolean қосу (тұрақты)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("{} тұрақты қосу.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Доменді Permissive етіп орнату (тек тексеру үшін) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} процесіне {} үшін {} рұқсаты берілмеді.", process, target, perm)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" {} порты SELinux саясатында анықталмаған.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} стандартты емес портта жұмыс істеу үшін порт контекстін қосуды қажет етеді.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" {} жазу рұқсаты берілмеді.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" {} белгісі жоғалуы мүмкін.", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" ls -dZ {} арқылы каталог таңбасын тексеріңіз. Таңба қате болса, алдымен restorecon қолданыңыз.", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" {} Boolean қосу мәселені шешуі мүмкін.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" {} доменінен {} операциясы саясатпен рұқсат етілмеген.", domain, perm)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{} сек бұрын", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{} мин бұрын", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{} сағ бұрын", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{} күн бұрын", n) }
pub const LABEL_FIRST_SEEN: &str = "Алғаш пайда болу";
pub const LABEL_LAST_SEEN:  &str = "Соңғы пайда болу";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Ескерту: локаль UTF-8 болмауы мүмкін (LANG={}).\n\
         Таңбалар дұрыс көрсетілмесе, LANG=kk_KZ.UTF-8 орнатыңыз.",
        lang_val
    )
}

// ── Тәуелділіктерді тексеру ───────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Кейбір қажетті командалар табылмады:";
pub const WARN_MISSING_OPT_FTR: &str = "       Жоғарыдағы командаларды пайдаланатын функциялар жұмыс істемейді.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Қажетті командалар табылмады. seadmin іске қосылмайды:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (пакет: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Жоғарыдағы пакеттерді орнатып, қайталап көріңіз.\n\
  мыс. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  мыс. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Журнал шығысы ─────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin іске қосылды (журнал: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Журнал файлы ашылмады: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC жүктелді: {} жазба", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (абсолютті жол жоқ — restorecon/fcontext жасырылды)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC жүктеу қатесі: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Команда сәтті орындалды";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Аутентификация сәтсіз ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Команда сәтсіз:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux режимі: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow жасалды: {} жол, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} журнал жолы)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (кэштелген аутентификация)", cmd) }

// ── Команда қателері ──────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "audit.log оқуға рұқсат жоқ. adm тобына қосыңыз немесе sudo баптаңыз.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow қатесі: {}", stderr) }
