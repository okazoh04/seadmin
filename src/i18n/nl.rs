/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Voettekst hints ───────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Verplaats  Enter:Details  /:Filteren  r:Herladen  l:Log  q:Afsluiten";
pub const HINT_AVC_DETAIL:   &str = "A-F:Selecteer  Esc/←:Terug  Enter:Bevestigen";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Scrollen  Enter:Toepassen  Esc:Annuleren";
pub const HINT_AUTH:         &str = "Enter:Uitvoeren  Esc:Annuleren";

// ── Tabelkoppen ───────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Wanneer";
pub const COL_PROCESS:  &str = "Proces";
pub const COL_ACTION:   &str = "Actie";
pub const COL_TARGET:   &str = "Doel";
pub const COL_COUNT:    &str = "Aantal";
pub const COL_REMEDY:   &str = "Oplossing";

// ── Status / berichten ────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ AVC-log laden...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux is uitgeschakeld. Toegangsweigeringen worden niet geregistreerd.";
pub const NO_AVC:           &str = "Geen toegangsweigeringen";
pub const OP_COMPLETE:      &str = "Bewerking voltooid";
pub const IGNORED:          &str = "Toegevoegd aan negeringslijst";
pub const FILTER_LABEL:     &str = "/Filteren: ";

// ── Authenticatievenster ──────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Beheerdersauthenticatie";
pub const AUTH_CMD_LABEL:  &str = "  Opdracht:";
pub const AUTH_PW_LABEL:   &str = "  Wachtwoord:";
pub const AUTH_CANCEL_BTN: &str = "[ Annuleren (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Uitvoeren (Enter) ]";
pub const PW_WRONG:        &str = "Onjuist wachtwoord";

// ── Blokken detailscherm ──────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Oorzaakanalyse ";
pub const BLOCK_OPTIONS:  &str = " Oplossingsopties ";
pub const BLOCK_RAW_LOG:  &str = " Ruwe log (referentie)";

// ── Beleidscontrole ───────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Beleidscontrole (Enter:Toepassen  Esc:Annuleren)";
pub const POLICY_APPLY_DESC:   &str = "Het gegenereerde beleidsmodule op het systeem toepassen.";

// ── Oplossingsopties (statisch) ───────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Standaard bestandscontext herstellen (verloren labels repareren).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Aangepast beleidsmodule genereren en toepassen (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Automatisch beleid genereren met audit2allow. Controleer voor toepassing.";
pub const OPT_PERMISSIVE_DESC:     &str = "Weigeringen tijdelijk uitschakelen. Vermindert beveiliging; alleen voor onderzoek.";
pub const OPT_IGNORE_LABEL:        &str = "Niets doen / Toevoegen aan negeringslijst";
pub const OPT_IGNORE_DESC:         &str = "Dit item toevoegen aan de negeringslijst (alleen binnen het hulpprogramma).";

// ── Oorzaakanalyse (statisch) ─────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Niet-standaard pad vereist toevoeging van een fcontext-regel.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Restorecon uitvoeren om de standaardcontext te herstellen kan dit oplossen.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Een aangepast beleid moet worden gegenereerd met audit2allow.";

// ── Remedy-weergavenamen ──────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Poortcontext";
pub const REMEDY_FILE_CONTEXT:  &str = "Bestandscontext";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Aangepast beleid";

// ── Opmaakstrings ─────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Toegangsweigeringen  [Vandaag]  Onopgelost: {} / Totaal: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} AVC-vermeldingen geladen", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Opdracht mislukt: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Vergrendeld ({} seconden resterend)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Log  {} vermeldingen  ↑↓:Scrollen  l:Sluiten ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Poortcontext toevoegen  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("ssh_port_t-context toewijzen aan poort {} van {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Herstellen met restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("fcontext wijzigen  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Regel toevoegen om {} aan dit pad toe te wijzen. Voer restorecon uit na toepassing.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Boolean inschakelen (tijdelijk)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("{} inschakelen (wordt hersteld na herstart).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Boolean inschakelen (permanent)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("{} permanent inschakelen.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Domein instellen op Permissive (alleen onderzoek) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} werd {} geweigerd op {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Poort {} is niet gedefinieerd in het SELinux-beleid.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} heeft een poortcontext nodig om op een niet-standaard poort te werken.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Schrijftoegang tot {} werd geweigerd.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Het label op {} is mogelijk verwijderd.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Het inschakelen van Boolean {} kan dit oplossen.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" De bewerking {} van domein {} is niet toegestaan door het beleid.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}s geleden", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}m geleden", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}u geleden", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}d geleden", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Waarschuwing: Locale is mogelijk niet UTF-8 (LANG={}).\n\
         Stel LANG=nl_NL.UTF-8 in als tekens niet correct worden weergegeven.",
        lang_val
    )
}

// ── Afhankelijkheidscontrole ───────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Sommige vereiste opdrachten zijn niet gevonden:";
pub const WARN_MISSING_OPT_FTR: &str = "       Functies die de bovenstaande opdrachten gebruiken, werken niet.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Vereiste opdrachten niet gevonden. Kan seadmin niet starten:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (pakket: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Installeer de bovenstaande pakketten en probeer opnieuw.\n\
  bijv. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  bijv. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Loguitvoer ────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin gestart (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Logbestand kon niet worden geopend: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC geladen: {} vermeldingen", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (geen absoluut pad — restorecon/fcontext verborgen)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC laden mislukt: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Opdracht geslaagd";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Verificatie mislukt ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Opdracht mislukt:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux-modus: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow gegenereerd: {} regels, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} logregels als invoer)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (gecachede verificatie)", cmd) }

// ── Opdrachtfouten ────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Geen toestemming om audit.log te lezen. Voeg uzelf toe aan de adm-groep of configureer sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow mislukt: {}", stderr) }
