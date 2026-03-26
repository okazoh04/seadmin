/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Suggerimenti a piè di pagina ──────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Sposta  Enter:Dettagli  /:Filtra  r:Ricarica  m:Moduli  l:Log  q:Esci";
pub const HINT_AVC_DETAIL:   &str = "A-F:Seleziona  Esc/←:Indietro  Enter:Conferma";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Scorri  Enter:Applica  Esc:Annulla";
pub const HINT_AUTH:         &str = "Enter:Esegui  Esc:Annulla";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Sposta  d:Elimina  a:Alterna tutto/pers.  Esc:Indietro";
pub const MODULE_LIST_AUTH_DESC: &str = "Sono necessari i privilegi di amministratore per elencare i moduli. Verrà caricato automaticamente dopo l'autenticazione.";
pub const HINT_MODULE_DETAIL:    &str = "↑↓/jk:Scorri  Esc:Indietro";
pub const MODULE_DETAIL_AUTH_DESC:  &str = "Sono necessari i privilegi di amministratore per i dettagli del modulo. Verrà caricato automaticamente dopo l'autenticazione.";
pub const MODULE_DETAIL_TITLE:      &str = " Dettagli modulo ";
pub const MODULE_DETAIL_CIL_TITLE:  &str = " Regole CIL ";

// ── Intestazioni tabella ──────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Quando";
pub const COL_PROCESS:  &str = "Processo";
pub const COL_ACTION:   &str = "Azione";
pub const COL_TARGET:   &str = "Destinazione";
pub const COL_COUNT:    &str = "Conteggio";
pub const COL_REMEDY:       &str = "Soluzione";
pub const COL_PRIORITY:     &str = "Priorità";
pub const COL_MODULE_NAME:  &str = "Modulo";

// ── Stato / messaggi ──────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Caricamento log AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux è disabilitato. I rifiuti di accesso non verranno registrati.";
pub const NO_AVC:           &str = "Nessun rifiuto di accesso";
pub const OP_COMPLETE:      &str = "Operazione completata";
pub const IGNORED:          &str = "Aggiunto alla lista ignorati";
pub const FILTER_LABEL:     &str = "/Filtra: ";

// ── Finestra di autenticazione ────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Autenticazione amministratore";
pub const AUTH_CMD_LABEL:  &str = "  Comando:";
pub const AUTH_PW_LABEL:   &str = "  Password:";
pub const AUTH_CANCEL_BTN: &str = "[ Annulla (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Esegui (Enter) ]";
pub const PW_WRONG:        &str = "Password errata";

// ── Blocchi schermata dettagli ────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Analisi causa ";
pub const BLOCK_OPTIONS:  &str = " Opzioni soluzione ";
pub const BLOCK_RAW_LOG:  &str = " Log grezzo (riferimento)";

// ── Revisione criterio ────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Revisione criterio (Enter:Applica  Esc:Annulla)";
pub const POLICY_APPLY_DESC:   &str = "Applica il modulo criterio generato al sistema.";

// ── Opzioni soluzione (statiche) ──────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Provare prima questo. Ripristina il contesto file predefinito (ripara etichette perse).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Genera e applica modulo criterio personalizzato (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Genera automaticamente un criterio con audit2allow. Se il percorso è noto, premere prima P.";
pub const OPT_PERMISSIVE_DESC:     &str = "⚠ Disabilita tutti i rifiuti del dominio. Rischio di sicurezza elevato. Solo per indagini.";
pub const OPT_IGNORE_LABEL:        &str = "Non fare nulla / Aggiungi alla lista ignorati";
pub const OPT_IGNORE_DESC:         &str = "Aggiungi questa voce alla lista ignorati (solo nello strumento).";

// ── Analisi causa (statica) ───────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Il percorso non standard richiede l'aggiunta di una regola fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Eseguire restorecon per ripristinare il contesto predefinito può risolvere il problema.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " È necessario generare un criterio personalizzato con audit2allow.";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " * Percorso sconosciuto. Premere P per specificare il percorso e vedere la soluzione ottimale.";
pub const PATH_INPUT_TITLE:  &str = " Inserire il percorso della directory";
pub const PATH_INPUT_PROMPT: &str = " Inserire il percorso assoluto (es: /var/log/myapp)";
pub const PATH_INPUT_HINT:   &str = " Enter: Conferma  Esc: Annulla";
pub const OPT_PATH_INPUT_LABEL: &str = "Inserire percorso assoluto per abilitare restorecon/fcontext";
pub const OPT_PATH_INPUT_DESC:  &str = "Percorso sconosciuto — opzioni A/B non disponibili. Inserire il percorso assoluto per visualizzare i passi di correzione etichetta (restorecon / semanage fcontext).";

// ── Nomi Remedy ───────────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Contesto porta";
pub const REMEDY_FILE_CONTEXT:  &str = "Contesto file";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Criterio personalizzato";

// ── Stringhe di formato ───────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Rifiuti di accesso  [Oggi]  Non risolti: {} / Totale: {} ", unresolved, total)
}
pub fn module_list_title(custom: usize, total: usize, show_all: bool) -> String {
    if show_all {
        format!(" Moduli criterio  {} totale ", total)
    } else {
        format!(" Moduli personalizzati  {} / {} totale ", custom, total)
    }
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Rimuovere il modulo criterio '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Modulo '{}' rimosso.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} voci AVC caricate", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Comando fallito: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Bloccato ({} secondi rimanenti)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Log  {} voci  ↑↓:Scorri  Esc:Chiudi ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Aggiungi contesto porta  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Assegna contesto ssh_port_t alla porta {} di {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Ripara con restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Cambia fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Aggiunge regola per assegnare {} a questo percorso ed esegue restorecon automaticamente.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Abilita Boolean (temporaneo)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Abilita {} (si ripristina al riavvio).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Abilita Boolean (permanente)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Abilita {} in modo permanente.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Imposta dominio su Permissive (solo indagine) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" A {} è stato negato {} su {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" La porta {} non è definita nel criterio SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} richiede un contesto porta per operare su una porta non standard.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" L'accesso in scrittura a {} è stato negato.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" L'etichetta su {} potrebbe essere stata rimossa.", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" Verificare l'etichetta della directory con: ls -dZ {}. Se errata, provare prima restorecon.", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Abilitare il Boolean {} potrebbe risolvere il problema.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" L'operazione {} del dominio {} non è consentita dal criterio.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}s fa", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}m fa", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}h fa", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}g fa", n) }
pub const LABEL_FIRST_SEEN: &str = "Prima occorrenza";
pub const LABEL_LAST_SEEN:  &str = "Ultima occorrenza";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Attenzione: Il locale potrebbe non essere UTF-8 (LANG={}).\n\
         Imposta LANG=it_IT.UTF-8 se i caratteri non vengono visualizzati correttamente.",
        lang_val
    )
}

// ── Verifica dipendenze ───────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Alcuni comandi necessari non sono stati trovati:";
pub const WARN_MISSING_OPT_FTR: &str = "       Le funzioni che utilizzano i comandi sopra non funzioneranno.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Comandi obbligatori non trovati. Impossibile avviare seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (pacchetto: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Installare i pacchetti sopra indicati e riprovare.\n\
  es. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  es. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Output log ────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin avviato (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Impossibile aprire il file di log: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC caricati: {} voci", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (nessun percorso assoluto — restorecon/fcontext nascosti)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] Caricamento AVC fallito: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Comando riuscito";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Autenticazione fallita ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Comando fallito:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] Modalità SELinux: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow generato: {} righe, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} righe di log come input)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (autenticazione in cache)", cmd) }

// ── Errori di comando ─────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Nessuna autorizzazione per leggere audit.log. Aggiungiti al gruppo adm o configura sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow non riuscito: {}", stderr) }
