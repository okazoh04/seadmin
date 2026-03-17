// ── Төменгі бөліктегі кеңестер ────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Жылжыту  Enter:Толық  /:Сүзгі  r:Жаңарту  l:Журнал  q:Шығу";
pub const HINT_AVC_DETAIL:   &str = "A-F:Іс-әрекет  Esc/←:Артқа  Enter:Растау";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Айналдыру  Enter:Қолдану  Esc:Болдырмау";
pub const HINT_AUTH:         &str = "Enter:Орындау  Esc:Болдырмау";

// ── Кесте тақырыптары ─────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Уақыт";
pub const COL_PROCESS:  &str = "Процесс";
pub const COL_ACTION:   &str = "Іс-әрекет";
pub const COL_TARGET:   &str = "Нысан";
pub const COL_COUNT:    &str = "Саны";
pub const COL_REMEDY:   &str = "Шешім";

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
pub const OPT_RESTORECON_DESC:     &str = "Файлдың әдепкі контекстін қалпына келтіру (жоғалған белгілерді түзету).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Теңшелген саясат модулін жасау және қолдану (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "audit2allow арқылы саясатты автоматты жасау. Қолданар алдында тексеруге болады.";
pub const OPT_PERMISSIVE_DESC:     &str = "Бас тартуларды уақытша өшіру. Қауіпсіздікті төмендетеді; тек тексеру үшін қолданыңыз.";
pub const OPT_IGNORE_LABEL:        &str = "Ешнәрсе жасамау / Еленбеу тізіміне қосу";
pub const OPT_IGNORE_DESC:         &str = "Бұл жазбаны еленбеу тізіміне қосу (тек құрал ішінде).";

// ── Себеп талдауы (статикалық) ────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Стандартты емес жол fcontext ережесін қосуды қажет етеді.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " restorecon арқылы әдепкі контекстті қалпына келтіру мәселені шешуі мүмкін.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " audit2allow арқылы теңшелген саясат жасау қажет.";

// ── Remedy атаулары ───────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Порт қосу";
pub const REMEDY_FILE_CONTEXT:  &str = "fcontext өзгерту";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Теңшелген саясат";

// ── Пішімдеу жолдары ──────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Кіруден бас тарту  [Бүгін]  Өңделмеген: {} / Барлығы: {} ", unresolved, total)
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
    format!(" Журнал  {} жазба  ↑↓:Айналдыру  l:Жабу ", total)
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
    format!("fcontext өзгерту  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Осы жолға {} тағайындайтын ереже қосу. Қолданғаннан кейін restorecon іске қосыңыз.", file_type)
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
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Ескерту: локаль UTF-8 болмауы мүмкін (LANG={}).\n\
         Таңбалар дұрыс көрсетілмесе, LANG=kk_KZ.UTF-8 орнатыңыз.",
        lang_val
    )
}
