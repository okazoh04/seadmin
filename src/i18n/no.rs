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
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Flytt  Enter:Detaljer  /:Filtrer  r:Last inn igjen  l:Logg  q:Avslutt";
pub const HINT_AVC_DETAIL:   &str = "A-F:Velg  Esc/←:Tilbake  Enter:Bekreft";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Rull  Enter:Bruk  Esc:Avbryt";
pub const HINT_AUTH:         &str = "Enter:Kjør  Esc:Avbryt";

// ── Tabelloverskrifter ────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Når";
pub const COL_PROCESS:  &str = "Prosess";
pub const COL_ACTION:   &str = "Handling";
pub const COL_TARGET:   &str = "Mål";
pub const COL_COUNT:    &str = "Antall";
pub const COL_REMEDY:   &str = "Løsning";

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
pub const OPT_RESTORECON_DESC:     &str = "Gjenopprett standard filkontekst (reparer tapte etiketter).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Generer og bruk tilpasset policymodul (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Generer policy automatisk med audit2allow. Gjennomgå før bruk.";
pub const OPT_PERMISSIVE_DESC:     &str = "Deaktiver nektelser midlertidig. Reduserer sikkerheten; bruk kun til undersøkelse.";
pub const OPT_IGNORE_LABEL:        &str = "Ikke gjør noe / Legg til ignoreringsliste";
pub const OPT_IGNORE_DESC:         &str = "Legg til denne oppføringen i ignoreringslisten (kun inne i verktøyet).";

// ── Årsaksanalyse (statisk) ───────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Ikke-standard sti krever tillegg av en fcontext-regel.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Å kjøre restorecon for å gjenopprette standardkonteksten kan løse dette.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " En tilpasset policy må genereres med audit2allow.";

// ── Remedy-visningsnavn ───────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Portkontekst";
pub const REMEDY_FILE_CONTEXT:  &str = "Filkontekst";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Tilpasset policy";

// ── Formatstrenger ────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Tilgangsnektelser  [I dag]  Uløste: {} / Totalt: {} ", unresolved, total)
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
    format!(" Logg  {} oppføringer  ↑↓:Rull  l:Lukk ", total)
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
    format!("Endre fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Legg til regel for å tilordne {} til denne stien. Kjør restorecon etter bruk.", file_type)
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
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Advarsel: Locale er kanskje ikke UTF-8 (LANG={}).\n\
         Angi LANG=nb_NO.UTF-8 hvis tegn ikke vises riktig.",
        lang_val
    )
}
