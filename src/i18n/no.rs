/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Bunntekst-hint ────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Flytt  Enter:Detaljer  /:Filtrer  r:Last inn igjen  m:Moduler  l:Logg  q:Avslutt";
pub const HINT_AVC_DETAIL:   &str = "A-F:Velg  Esc/←:Tilbake  Enter:Bekreft";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Rull  Enter:Bruk  Esc:Avbryt";
pub const HINT_AUTH:         &str = "Enter:Kjør  Esc:Avbryt";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Flytt  d:Slett  Esc:Tilbake";

// ── Tabelloverskrifter ────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Når";
pub const COL_PROCESS:  &str = "Prosess";
pub const COL_ACTION:   &str = "Handling";
pub const COL_TARGET:   &str = "Mål";
pub const COL_COUNT:    &str = "Antall";
pub const COL_REMEDY:       &str = "Løsning";
pub const COL_PRIORITY:     &str = "Prioritet";
pub const COL_MODULE_NAME:  &str = "Modul";

// ── Status / meldinger ────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Laster AVC-logg...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux er deaktivert. Tilgangsnektelser vil ikke bli registrert.";
pub const NO_AVC:           &str = "Ingen tilgangsnektelser";
pub const OP_COMPLETE:      &str = "Operasjon fullført";
pub const IGNORED:          &str = "Lagt til ignoreringsliste";
pub const FILTER_LABEL:     &str = "/Filtrer: ";

// ── Autentiseringsvindu ───────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Administratorautentisering";
pub const AUTH_CMD_LABEL:  &str = "  Kommando:";
pub const AUTH_PW_LABEL:   &str = "  Passord:";
pub const AUTH_CANCEL_BTN: &str = "[ Avbryt (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Kjør (Enter) ]";
pub const PW_WRONG:        &str = "Feil passord";

// ── Blokker på detaljskjermen ─────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Årsaksanalyse ";
pub const BLOCK_OPTIONS:  &str = " Løsningsalternativer ";
pub const BLOCK_RAW_LOG:  &str = " Rå logg (referanse)";

// ── Policygjennomgang ─────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Policygjennomgang (Enter:Bruk  Esc:Avbryt)";
pub const POLICY_APPLY_DESC:   &str = "Bruk den genererte policymodulen på systemet.";

// ── Løsningsalternativer (statiske) ──────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Prøv dette først. Gjenopprett standard filkontekst (reparer tapte etiketter).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Generer og bruk tilpasset policymodul (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Generer policy automatisk med audit2allow. Hvis søkestien er kjent, trykk P først.";
pub const OPT_PERMISSIVE_DESC:     &str = "⚠ Deaktiverer alle nektelser for domenet. Stor sikkerhetsrisiko. Bruk kun til undersøkelse.";
pub const OPT_IGNORE_LABEL:        &str = "Ikke gjør noe / Legg til ignoreringsliste";
pub const OPT_IGNORE_DESC:         &str = "Legg til denne oppføringen i ignoreringslisten (kun inne i verktøyet).";

// ── Årsaksanalyse (statisk) ───────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Ikke-standard sti krever tillegg av en fcontext-regel.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Å kjøre restorecon for å gjenopprette standardkonteksten kan løse dette.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " En tilpasset policy må genereres med audit2allow.";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " * Søkesti ukjent. Trykk P for å angi søkestien og se den beste løsningen.";
pub const PATH_INPUT_TITLE:  &str = " Angi katalogsøkesti";
pub const PATH_INPUT_PROMPT: &str = " Skriv inn absolutt søkesti (f.eks. /var/log/myapp)";
pub const PATH_INPUT_HINT:   &str = " Enter: Bekreft  Esc: Avbryt";
pub const OPT_PATH_INPUT_LABEL: &str = "Angi absolutt sti for å aktivere restorecon/fcontext";
pub const OPT_PATH_INPUT_DESC:  &str = "Sti ukjent — alternativene A/B kan ikke vises. Angi absolutt sti for å vise trinn for etikettkorrigering (restorecon / semanage fcontext).";

// ── Remedy-visningsnavn ───────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Portkontekst";
pub const REMEDY_FILE_CONTEXT:  &str = "Filkontekst";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Tilpasset policy";

// ── Formatstrenger ────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Tilgangsnektelser  [I dag]  Uløste: {} / Totalt: {} ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" Policymoduler  {} moduler ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Slett policymodul '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Modul '{}' slettet.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} AVC-oppføringer lastet", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Kommando mislyktes: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Utestengt ({} sekunder gjenstår)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Logg  {} oppføringer  ↑↓:Rull  Esc:Lukk ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Legg til portkontekst  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Tildel ssh_port_t-kontekst til port {} av {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Reparer med restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Endre fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Legg til regel for å tilordne {} til denne stien og kjør restorecon automatisk.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Aktiver Boolean (midlertidig)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Aktiver {} (tilbakestilles ved omstart).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Aktiver Boolean (permanent)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Aktiver {} permanent.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Sett domene til Permissive (kun undersøkelse) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} ble nektet {} på {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Port {} er ikke definert i SELinux-policyen.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} trenger en portkontekst for å fungere på en ikke-standard port.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Skrivetilgang til {} ble nektet.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" Etiketten på {} kan ha blitt fjernet.", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" Sjekk katalogens etikett med: ls -dZ {}. Hvis feil etikett, prøv restorecon først.", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Å aktivere Boolean {} kan løse dette.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" Handlingen {} fra domene {} er ikke tillatt av policyen.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("for {}s siden", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("for {}m siden", n) }
pub fn elapsed_hours(n: u64) -> String { format!("for {}t siden", n) }
pub fn elapsed_days(n: u64)  -> String { format!("for {}d siden", n) }
pub const LABEL_FIRST_SEEN: &str = "Første forekomst";
pub const LABEL_LAST_SEEN:  &str = "Siste forekomst";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Advarsel: Locale er kanskje ikke UTF-8 (LANG={}).\n\
         Angi LANG=nb_NO.UTF-8 hvis tegn ikke vises riktig.",
        lang_val
    )
}

// ── Avhengighetssjekk ─────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Noen nødvendige kommandoer ble ikke funnet:";
pub const WARN_MISSING_OPT_FTR: &str = "       Funksjoner som bruker kommandoene ovenfor vil ikke fungere.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Nødvendige kommandoer ikke funnet. Kan ikke starte seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (pakke: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Installer pakkene ovenfor og prøv igjen.\n\
  f.eks. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  f.eks. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Loggutdata ────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin startet (logg: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Kunne ikke åpne loggfilen: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC lastet: {} oppføringer", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (ingen absolutt sti — restorecon/fcontext skjult)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC-lasting mislyktes: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Kommando vellykket";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Autentisering mislyktes ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Kommando mislyktes:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux-modus: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow generert: {} linjer, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} logglinjer som inndata)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (bufret autentisering)", cmd) }

// ── Kommandofeil ──────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Ingen tillatelse til å lese audit.log. Legg deg til i adm-gruppen eller konfigurer sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow mislyktes: {}", stderr) }
