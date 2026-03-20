/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Подсказки в нижней строке ─────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Перемещение  Enter:Детали  /:Фильтр  r:Обновить  m:Модули  l:Журнал  q:Выход";
pub const HINT_AVC_DETAIL:   &str = "A-F:Выбор действия  Esc/←:Назад  Enter:Подтвердить";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Прокрутка  Enter:Применить  Esc:Отмена";
pub const HINT_AUTH:         &str = "Enter:Выполнить  Esc:Отмена";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Перемещение  d:Удалить  Esc:Назад";

// ── Заголовки таблицы ─────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Время";
pub const COL_PROCESS:  &str = "Процесс";
pub const COL_ACTION:   &str = "Действие";
pub const COL_TARGET:   &str = "Цель";
pub const COL_COUNT:    &str = "Количество";
pub const COL_REMEDY:       &str = "Решение";
pub const COL_PRIORITY:     &str = "Приоритет";
pub const COL_MODULE_NAME:  &str = "Модуль";

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
pub fn module_list_title(count: usize) -> String {
    format!(" Модули политики  {} шт. ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Удалить модуль политики '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Модуль '{}' удалён.", name)
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
    format!("Изменить fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Добавить правило назначения {} этому пути и автоматически выполнить restorecon.", file_type)
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
pub const LABEL_FIRST_SEEN: &str = "Первое появление";
pub const LABEL_LAST_SEEN:  &str = "Последнее появление";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Предупреждение: локаль может быть не UTF-8 (LANG={}).\n\
         Установите LANG=ru_RU.UTF-8, если символы отображаются некорректно.",
        lang_val
    )
}

// ── Вывод проверки зависимостей ───────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Некоторые необходимые команды не найдены:";
pub const WARN_MISSING_OPT_FTR: &str = "       Функции, использующие эти команды, не будут работать.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Необходимые команды не найдены. Невозможно запустить seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (пакет: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Установите указанные пакеты и повторите попытку.\n\
  напр. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  напр. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Вывод в журнал ────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin запущен (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Не удалось открыть файл журнала: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC загружено: {} записей", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (нет абсолютного пути — restorecon/fcontext скрыты)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] Ошибка загрузки AVC: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Команда выполнена успешно";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Ошибка аутентификации ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Ошибка выполнения команды:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] Режим SELinux: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow сгенерирован: {} строк, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} строк журнала)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (кэш аутентификации)", cmd) }

// ── Ошибки команд ─────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Нет прав на чтение audit.log. Добавьте себя в группу adm или настройте sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("Ошибка audit2allow: {}", stderr) }
