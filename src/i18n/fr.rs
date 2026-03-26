/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Conseils en bas de page ───────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Déplacer  Enter:Détails  /:Filtrer  r:Actualiser  m:Modules  l:Journal  q:Quitter";
pub const HINT_AVC_DETAIL:   &str = "A-F:Sélectionner  Esc/←:Retour  Enter:Confirmer";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Défiler  Enter:Appliquer  Esc:Annuler";
pub const HINT_AUTH:         &str = "Enter:Exécuter  Esc:Annuler";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Déplacer  d:Supprimer  a:Basculer tout/pers.  Esc:Retour";
pub const MODULE_LIST_AUTH_DESC: &str = "Des privilèges d'administrateur sont requis pour lister les modules. Chargement automatique après authentification.";
pub const HINT_MODULE_DETAIL:    &str = "↑↓/jk:Défiler  Esc:Retour";
pub const MODULE_DETAIL_AUTH_DESC:  &str = "Des privilèges d'administrateur sont requis pour les détails du module. Chargement automatique après authentification.";
pub const MODULE_DETAIL_TITLE:      &str = " Détails du module ";
pub const MODULE_DETAIL_CIL_TITLE:  &str = " Règles CIL ";

// ── En-têtes de tableau ───────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Quand";
pub const COL_PROCESS:  &str = "Processus";
pub const COL_ACTION:   &str = "Action";
pub const COL_TARGET:   &str = "Cible";
pub const COL_COUNT:    &str = "Nombre";
pub const COL_REMEDY:       &str = "Solution";
pub const COL_PRIORITY:     &str = "Priorité";
pub const COL_MODULE_NAME:  &str = "Module";

// ── Statut / messages ─────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Chargement du journal AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux est désactivé. Les refus d'accès ne seront pas enregistrés.";
pub const NO_AVC:           &str = "Aucun refus d'accès";
pub const OP_COMPLETE:      &str = "Opération terminée";
pub const IGNORED:          &str = "Ajouté à la liste d'ignorés";
pub const FILTER_LABEL:     &str = "/Filtrer: ";

// ── Fenêtre d'authentification ────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Authentification administrateur";
pub const AUTH_CMD_LABEL:  &str = "  Commande :";
pub const AUTH_PW_LABEL:   &str = "  Mot de passe :";
pub const AUTH_CANCEL_BTN: &str = "[ Annuler (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Exécuter (Enter) ]";
pub const PW_WRONG:        &str = "Mot de passe incorrect";

// ── Blocs de l'écran de détails ───────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Analyse de cause ";
pub const BLOCK_OPTIONS:  &str = " Options de solution ";
pub const BLOCK_RAW_LOG:  &str = " Journal brut (référence)";

// ── Révision de politique ─────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Révision de politique (Enter:Appliquer  Esc:Annuler)";
pub const POLICY_APPLY_DESC:   &str = "Appliquer le module de politique généré au système.";

// ── Options de solution (statiques) ──────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Essayez d'abord ceci. Restaure le contexte de fichier par défaut (répare les étiquettes perdues).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Générer et appliquer un module de politique personnalisé (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Générer automatiquement une politique avec audit2allow. Si le chemin est connu, appuyez d'abord sur P.";
pub const OPT_PERMISSIVE_DESC:     &str = "⚠ Désactive toutes les refus du domaine. Risque majeur pour la sécurité. Uniquement pour l'investigation.";
pub const OPT_IGNORE_LABEL:        &str = "Ne rien faire / Ajouter à la liste d'ignorés";
pub const OPT_IGNORE_DESC:         &str = "Ajouter cette entrée à la liste d'ignorés (uniquement dans l'outil).";

// ── Analyse de cause (statique) ───────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Le chemin non standard nécessite l'ajout d'une règle fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Exécuter restorecon pour restaurer le contexte par défaut peut résoudre ce problème.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Une politique personnalisée doit être générée avec audit2allow.";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " * Chemin inconnu. Appuyez sur P pour spécifier le chemin et voir la meilleure correction.";
pub const PATH_INPUT_TITLE:  &str = " Saisir le chemin du répertoire";
pub const PATH_INPUT_PROMPT: &str = " Saisissez le chemin absolu (ex : /var/log/myapp)";
pub const PATH_INPUT_HINT:   &str = " Entrée: Confirmer  Échap: Annuler";
pub const OPT_PATH_INPUT_LABEL: &str = "Saisir le chemin absolu pour activer restorecon/fcontext";
pub const OPT_PATH_INPUT_DESC:  &str = "Le chemin est inconnu — les options A/B ne peuvent pas être affichées. Saisissez le chemin absolu pour afficher les étapes de correction de contexte (restorecon / semanage fcontext).";

// ── Noms de Remedy ────────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Contexte de port";
pub const REMEDY_FILE_CONTEXT:  &str = "Contexte de fichier";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Politique personnalisée";

// ── Chaînes de format ─────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Refus d'accès  [Aujourd'hui]  Non résolus: {} / Total: {} ", unresolved, total)
}
pub fn module_list_title(custom: usize, total: usize, show_all: bool) -> String {
    if show_all {
        format!(" Modules de politique  {} total ", total)
    } else {
        format!(" Modules personnalisés  {} / {} total ", custom, total)
    }
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Supprimer le module de politique '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Module '{}' supprimé.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} entrées AVC chargées", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Échec de commande: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Verrouillé ({} secondes restantes)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Journal  {} entrées  ↑↓:Défiler  Esc:Fermer ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Ajouter contexte de port  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Attribuer le contexte ssh_port_t au port {} de {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Réparer avec restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Changer fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Ajouter une règle pour attribuer {} à ce chemin et exécuter restorecon automatiquement.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Activer Boolean (temporaire)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Activer {} (revient après redémarrage).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Activer Boolean (permanent)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Activer {} en permanence.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Définir le domaine en Permissive (investigation uniquement) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} s'est vu refuser {} sur {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" Le port {} n'est pas défini dans la politique SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} nécessite l'ajout d'un contexte de port pour fonctionner sur un port non standard.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" L'accès en écriture à {} a été refusé.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" L'étiquette sur {} a peut-être été supprimée.", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" Vérifiez l'étiquette du répertoire avec : ls -dZ {}. Si mal étiquetée, essayez restorecon en premier.", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Activer le Boolean {} peut résoudre ce problème.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" L'opération {} du domaine {} n'est pas autorisée par la politique.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("il y a {}s", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("il y a {}m", n) }
pub fn elapsed_hours(n: u64) -> String { format!("il y a {}h", n) }
pub fn elapsed_days(n: u64)  -> String { format!("il y a {}j", n) }
pub const LABEL_FIRST_SEEN: &str = "Première occurrence";
pub const LABEL_LAST_SEEN:  &str = "Dernière occurrence";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Avertissement: La locale n'est peut-être pas UTF-8 (LANG={}).\n\
         Définissez LANG=fr_FR.UTF-8 si les caractères ne s'affichent pas correctement.",
        lang_val
    )
}

// ── Vérification des dépendances ──────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] Certaines commandes requises sont introuvables:";
pub const WARN_MISSING_OPT_FTR: &str = "       Les fonctions utilisant ces commandes ne fonctionneront pas.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Commandes requises introuvables. Impossible de démarrer seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (paquet : {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Veuillez installer les paquets ci-dessus et réessayer.\n\
  ex. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  ex. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Sortie journal ────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin démarré (journal : {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] Impossible d'ouvrir le fichier journal : {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC chargé : {} entrées", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (aucun chemin absolu — restorecon/fcontext masqué)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] Échec du chargement AVC : {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Commande réussie";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Échec de l'authentification ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Échec de la commande :\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] Mode SELinux : {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow généré : {} lignes, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} lignes de journal en entrée)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (authentification en cache)", cmd) }

// ── Erreurs de commande ───────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Aucune permission de lire audit.log. Ajoutez-vous au groupe adm ou configurez sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("Échec d'audit2allow : {}", stderr) }
