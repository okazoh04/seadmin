// ── Подсказки в нижней строке ─────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Перемещение  Enter:Детали  /:Фильтр  r:Обновить  l:Журнал  q:Выход";
pub const HINT_AVC_DETAIL:   &str = "A-F:Выбор действия  Esc/←:Назад  Enter:Подтвердить";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Прокрутка  Enter:Применить  Esc:Отмена";
pub const HINT_AUTH:         &str = "Enter:Выполнить  Esc:Отмена";

// ── Заголовки таблицы ─────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Время";
pub const COL_PROCESS:  &str = "Процесс";
pub const COL_ACTION:   &str = "Действие";
pub const COL_TARGET:   &str = "Цель";
pub const COL_COUNT:    &str = "Количество";
pub const COL_REMEDY:   &str = "Решение";

// ── Статус / сообщения ────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Загрузка журнала AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux отключён. Отказы в доступе не будут записываться.";
pub const NO_AVC:           &str = "Отказов в доступе нет";
pub const OP_COMPLETE:      &str = "Операция завершена";
pub const IGNORED:          &str = "Добавлено в список игнорирования";
pub const FILTER_LABEL:     &str = "/Фильтр: ";

// ── Окно аутентификации ───────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Аутентификация администратора";
pub const AUTH_CMD_LABEL:  &str = "  Команда:";
pub const AUTH_PW_LABEL:   &str = "  Пароль:";
pub const AUTH_CANCEL_BTN: &str = "[ Отмена (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Выполнить (Enter) ]";
pub const PW_WRONG:        &str = "Неверный пароль";

// ── Блоки детального экрана ───────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Анализ причин ";
pub const BLOCK_OPTIONS:  &str = " Варианты решения ";
pub const BLOCK_RAW_LOG:  &str = " Необработанный журнал (справочно)";

// ── Проверка политики ─────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Проверка политики (Enter:Применить  Esc:Отмена)";
pub const POLICY_APPLY_DESC:   &str = "Применить сгенерированный модуль политики к системе.";

// ── Варианты решения (статические) ───────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Восстановить контекст файла по умолчанию (исправить потерянные метки).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Создать и применить пользовательский модуль политики (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Автоматически создать политику с audit2allow. Проверьте перед применением.";
pub const OPT_PERMISSIVE_DESC:     &str = "Временно отключить запреты. Снижает безопасность; используйте только для диагностики.";
pub const OPT_IGNORE_LABEL:        &str = "Ничего не делать / Добавить в список игнорирования";
pub const OPT_IGNORE_DESC:         &str = "Добавить запись в список игнорирования (только внутри инструмента).";

// ── Анализ причин (статический) ───────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Нестандартный путь требует добавления правила fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Запуск restorecon для восстановления контекста по умолчанию может решить проблему.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Необходимо создать пользовательскую политику с помощью audit2allow.";

// ── Отображаемые имена Remedy ─────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Добавить порт";
pub const REMEDY_FILE_CONTEXT:  &str = "Изменить fcontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Пользовательская политика";

// ── Строки форматирования ─────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Отказы в доступе  [Сегодня]  Необработанных: {} / Всего: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("Загружено {} записей AVC", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Ошибка команды: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Заблокировано (осталось {} сек.)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Журнал  {} записей  ↑↓:Прокрутка  l:Закрыть ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Добавить контекст порта  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Назначить контекст ssh_port_t порту {} протокола {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Восстановить с restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Изменить fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Добавить правило назначения {} этому пути. После применения выполните restorecon.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Включить Boolean (временно)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Включить {} (сбрасывается после перезагрузки).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Включить Boolean (постоянно)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Постоянно включить {}.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Установить домен в Permissive (только для диагностики) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" Процессу {} отказано в {} для {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Порт {} не определён в политике SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} требует добавления контекста порта для работы на нестандартном порту.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Запись в {} запрещена.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Метка на {} могла быть удалена.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Включение Boolean {} может решить проблему.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" Операция {} из домена {} не разрешена политикой.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{} с назад", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{} мин назад", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{} ч назад", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{} д назад", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Предупреждение: локаль может быть не UTF-8 (LANG={}).\n\
         Установите LANG=ru_RU.UTF-8, если символы отображаются некорректно.",
        lang_val
    )
}
