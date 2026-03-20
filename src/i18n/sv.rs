/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Sidfotsledtrådar ──────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Flytta  Enter:Detaljer  /:Filtrera  r:Ladda om  m:Moduler  l:Logg  q:Avsluta";
pub const HINT_AVC_DETAIL:   &str = "A-F:Välj  Esc/←:Tillbaka  Enter:Bekräfta";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Rulla  Enter:Tillämpa  Esc:Avbryt";
pub const HINT_AUTH:         &str = "Enter:Kör  Esc:Avbryt";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Flytta  d:Ta bort  Esc:Tillbaka";

// ── Tabellrubriker ────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "När";
pub const COL_PROCESS:  &str = "Process";
pub const COL_ACTION:   &str = "Åtgärd";
pub const COL_TARGET:   &str = "Mål";
pub const COL_COUNT:    &str = "Antal";
pub const COL_REMEDY:       &str = "Lösning";
pub const COL_PRIORITY:     &str = "Prioritet";
pub const COL_MODULE_NAME:  &str = "Modul";

// ── Status / meddelanden ──────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Laddar AVC-logg...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux är inaktiverat. Åtkomstnekanden registreras inte.";
pub const NO_AVC:           &str = "Inga åtkomstnekanden";
pub const OP_COMPLETE:      &str = "Åtgärd slutförd";
pub const IGNORED:          &str = "Tillagd i ignoreringslista";
pub const FILTER_LABEL:     &str = "/Filtrera: ";

// ── Autentiseringsfönster ─────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Administratörsautentisering";
pub const AUTH_CMD_LABEL:  &str = "  Kommando:";
pub const AUTH_PW_LABEL:   &str = "  Lösenord:";
pub const AUTH_CANCEL_BTN: &str = "[ Avbryt (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Kör (Enter) ]";
pub const PW_WRONG:        &str = "Fel lösenord";

// ── Block på detaljskärmen ────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Orsaksanalys ";
pub const BLOCK_OPTIONS:  &str = " Lösningsalternativ ";
pub const BLOCK_RAW_LOG:  &str = " Rå logg (referens)";

// ── Principgranskning ─────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Principgranskning (Enter:Tillämpa  Esc:Avbryt)";
pub const POLICY_APPLY_DESC:   &str = "Tillämpa den genererade principmodulen på systemet.";

// ── Lösningsalternativ (statiska) ─────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Återställ standardfilkontext (reparera borttagna etiketter).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Generera och tillämpa anpassad principmodul (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Generera princip automatiskt med audit2allow. Granska innan tillämpning.";
pub const OPT_PERMISSIVE_DESC:     &str = "Inaktivera nekanden tillfälligt. Minskar säkerheten; använd endast för undersökning.";
pub const OPT_IGNORE_LABEL:        &str = "Gör inget / Lägg till i ignoreringslista";
pub const OPT_IGNORE_DESC:         &str = "Lägg till denna post i ignoreringslistan (endast inom verktyget).";

// ── Orsaksanalys (statisk) ────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Icke-standardsökväg kräver tillägg av en fcontext-regel.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Att köra restorecon för att återställa standardkontexten kan lösa detta.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " En anpassad princip måste genereras med audit2allow.";

// ── Remedy-visningsnamn ───────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Portkontext";
pub const REMEDY_FILE_CONTEXT:  &str = "Filkontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Anpassad princip";

// ── Formatsträngar ────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Åtkomstnekanden  [Idag]  Olösta: {} / Totalt: {} ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" Principmoduler  {} moduler ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Ta bort principmodul '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Modul '{}' borttagen.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} AVC-poster laddade", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Kommando misslyckades: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Utelåst ({} sekunder kvar)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Logg  {} poster  ↑↓:Rulla  Esc:Stäng ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Lägg till portkontext  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Tilldela ssh_port_t-kontext till port {} av {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Reparera med restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Ändra fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Lägg till regel för att tilldela {} till denna sökväg och kör restorecon automatiskt.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Aktivera Boolean (tillfälligt)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Aktivera {} (återställs vid omstart).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Aktivera Boolean (permanent)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Aktivera {} permanent.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Ange domän som Permissive (endast undersökning) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} nekades {} på {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Port {} är inte definierad i SELinux-principen.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} behöver en portkontext för att fungera på en icke-standardport.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Skrivåtkomst till {} nekades.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Etiketten på {} kan ha tagits bort.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Att aktivera Boolean {} kan lösa detta.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" Åtgärden {} från domän {} är inte tillåten av principen.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("för {}s sedan", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("för {}m sedan", n) }
pub fn elapsed_hours(n: u64) -> String { format!("för {}h sedan", n) }
pub fn elapsed_days(n: u64)  -> String { format!("för {}d sedan", n) }
pub const LABEL_FIRST_SEEN: &str = "Första förekomst";
pub const LABEL_LAST_SEEN:  &str = "Senaste förekomst";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Varning: Locale kanske inte är UTF-8 (LANG={}).\n\
         Ange LANG=sv_SE.UTF-8 om tecken inte visas korrekt.",
        lang_val
    )
}

// ── Beroendekontrollen ────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Vissa nödvändiga kommandon hittades inte:";
pub const WARN_MISSING_OPT_FTR: &str = "       Funktioner som använder ovanstående kommandon fungerar inte.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Nödvändiga kommandon hittades inte. Kan inte starta seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (paket: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Installera ovanstående paket och försök igen.\n\
  t.ex. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  t.ex. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Loggutdata ────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin startad (logg: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Det gick inte att öppna loggfilen: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC inläst: {} poster", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (ingen absolut sökväg — restorecon/fcontext dold)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC-inläsning misslyckades: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Kommando lyckades";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Autentisering misslyckades ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Kommando misslyckades:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux-läge: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow genererad: {} rader, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} loggrader som inmatning)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (cachad autentisering)", cmd) }

// ── Kommandofel ───────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Ingen behörighet att läsa audit.log. Lägg till dig i adm-gruppen eller konfigurera sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow misslyckades: {}", stderr) }
