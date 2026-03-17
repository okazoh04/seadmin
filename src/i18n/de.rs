// ── Fußzeilen-Hinweise ────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Bewegen  Enter:Details  /:Filtern  r:Neu laden  l:Log  q:Beenden";
pub const HINT_AVC_DETAIL:   &str = "A-F:Auswählen  Esc/←:Zurück  Enter:Bestätigen";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Scrollen  Enter:Anwenden  Esc:Abbrechen";
pub const HINT_AUTH:         &str = "Enter:Ausführen  Esc:Abbrechen";

// ── Tabellenüberschriften ─────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Wann";
pub const COL_PROCESS:  &str = "Prozess";
pub const COL_ACTION:   &str = "Aktion";
pub const COL_TARGET:   &str = "Ziel";
pub const COL_COUNT:    &str = "Anzahl";
pub const COL_REMEDY:   &str = "Lösung";

// ── Status / Meldungen ────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ AVC-Protokoll wird geladen...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux ist deaktiviert. Zugriffsverweigerungen werden nicht aufgezeichnet.";
pub const NO_AVC:           &str = "Keine Zugriffsverweigerungen";
pub const OP_COMPLETE:      &str = "Vorgang abgeschlossen";
pub const IGNORED:          &str = "Zur Ignorierliste hinzugefügt";
pub const FILTER_LABEL:     &str = "/Filtern: ";

// ── Authentifizierungsfenster ─────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Administrator-Authentifizierung";
pub const AUTH_CMD_LABEL:  &str = "  Befehl:";
pub const AUTH_PW_LABEL:   &str = "  Passwort:";
pub const AUTH_CANCEL_BTN: &str = "[ Abbrechen (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Ausführen (Enter) ]";
pub const PW_WRONG:        &str = "Falsches Passwort";

// ── Blöcke des Detailbildschirms ──────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Ursachenanalyse ";
pub const BLOCK_OPTIONS:  &str = " Lösungsoptionen ";
pub const BLOCK_RAW_LOG:  &str = " Rohprotokoll (Referenz)";

// ── Richtlinienüberprüfung ────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Richtlinienüberprüfung (Enter:Anwenden  Esc:Abbrechen)";
pub const POLICY_APPLY_DESC:   &str = "Das generierte Richtlinienmodul auf das System anwenden.";

// ── Lösungsoptionen (statisch) ────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Standard-Dateikontext wiederherstellen (verlorene Bezeichnungen reparieren).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Benutzerdefiniertes Richtlinienmodul generieren und anwenden (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Richtlinie automatisch mit audit2allow generieren. Vor der Anwendung überprüfen.";
pub const OPT_PERMISSIVE_DESC:     &str = "Verweigerungen vorübergehend deaktivieren. Reduziert die Sicherheit; nur zur Untersuchung.";
pub const OPT_IGNORE_LABEL:        &str = "Nichts tun / Zur Ignorierliste hinzufügen";
pub const OPT_IGNORE_DESC:         &str = "Diesen Eintrag zur Ignorierliste hinzufügen (nur intern im Tool).";

// ── Ursachenanalyse (statisch) ────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Nicht-standardmäßiger Pfad erfordert das Hinzufügen einer fcontext-Regel.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Das Ausführen von restorecon zur Wiederherstellung des Standardkontexts kann dies beheben.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Eine benutzerdefinierte Richtlinie muss mit audit2allow generiert werden.";

// ── Remedy-Anzeigenamen ───────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Port-Kontext";
pub const REMEDY_FILE_CONTEXT:  &str = "Datei-Kontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Benutzerdefinierte Richtlinie";

// ── Formatzeichenketten ───────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Zugriffsverweigerungen  [Heute]  Ungelöst: {} / Gesamt: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} AVC-Einträge geladen", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Befehl fehlgeschlagen: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Gesperrt ({} Sekunden verbleibend)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Protokoll  {} Einträge  ↑↓:Scrollen  l:Schließen ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Port-Kontext hinzufügen  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("ssh_port_t-Kontext Port {} von {} zuweisen.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Mit restorecon reparieren  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("fcontext ändern  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Regel hinzufügen, um {} diesem Pfad zuzuweisen. Nach der Anwendung restorecon ausführen.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Boolean aktivieren (temporär)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("{} aktivieren (wird nach Neustart zurückgesetzt).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Boolean aktivieren (dauerhaft)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("{} dauerhaft aktivieren.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Domäne auf Permissive setzen (nur Untersuchung) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} wurde {} auf {} verweigert.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Port {} ist in der SELinux-Richtlinie nicht definiert.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} benötigt einen Port-Kontext, um auf einem nicht-standardmäßigen Port zu arbeiten.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Schreibzugriff auf {} wurde verweigert.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Die Bezeichnung auf {} wurde möglicherweise entfernt.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Das Aktivieren des Boolean {} kann dies beheben.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" Die Operation {} der Domäne {} ist durch die Richtlinie nicht erlaubt.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("vor {}s", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("vor {}m", n) }
pub fn elapsed_hours(n: u64) -> String { format!("vor {}h", n) }
pub fn elapsed_days(n: u64)  -> String { format!("vor {}T", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Warnung: Locale ist möglicherweise nicht UTF-8 (LANG={}).\n\
         Setzen Sie LANG=de_DE.UTF-8, wenn Zeichen nicht korrekt angezeigt werden.",
        lang_val
    )
}
