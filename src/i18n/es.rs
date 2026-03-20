/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Sugerencias en el pie ─────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Mover  Enter:Detalle  /:Filtrar  r:Recargar  m:Módulos  l:Log  q:Salir";
pub const HINT_AVC_DETAIL:   &str = "A-F:Seleccionar  Esc/←:Volver  Enter:Confirmar";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Desplazar  Enter:Aplicar  Esc:Cancelar";
pub const HINT_AUTH:         &str = "Enter:Ejecutar  Esc:Cancelar";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:Mover  d:Eliminar  Esc:Volver";

// ── Encabezados de tabla ──────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Cuándo";
pub const COL_PROCESS:  &str = "Proceso";
pub const COL_ACTION:   &str = "Acción";
pub const COL_TARGET:   &str = "Destino";
pub const COL_COUNT:    &str = "Veces";
pub const COL_REMEDY:       &str = "Solución";
pub const COL_PRIORITY:     &str = "Prioridad";
pub const COL_MODULE_NAME:  &str = "Módulo";

// ── Estado / mensajes ─────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Cargando registro AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux está desactivado. Las denegaciones de acceso no serán registradas.";
pub const NO_AVC:           &str = "No hay denegaciones de acceso";
pub const OP_COMPLETE:      &str = "Operación completada";
pub const IGNORED:          &str = "Añadido a la lista de ignorados";
pub const FILTER_LABEL:     &str = "/Filtrar: ";

// ── Ventana de autenticación ──────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Autenticación de administrador";
pub const AUTH_CMD_LABEL:  &str = "  Comando:";
pub const AUTH_PW_LABEL:   &str = "  Contraseña:";
pub const AUTH_CANCEL_BTN: &str = "[ Cancelar (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Ejecutar (Enter) ]";
pub const PW_WRONG:        &str = "Contraseña incorrecta";

// ── Bloques de pantalla de detalle ────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Análisis de causa ";
pub const BLOCK_OPTIONS:  &str = " Opciones de solución ";
pub const BLOCK_RAW_LOG:  &str = " Log sin procesar (referencia)";

// ── Revisión de política ──────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Revisión de política (Enter:Aplicar  Esc:Cancelar)";
pub const POLICY_APPLY_DESC:   &str = "Aplicar el módulo de política generado al sistema.";

// ── Opciones de solución (estáticas) ─────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Restaurar el contexto de archivo predeterminado (reparar etiquetas perdidas).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Generar y aplicar módulo de política personalizado (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Generar automáticamente una política con audit2allow. Revise antes de aplicar.";
pub const OPT_PERMISSIVE_DESC:     &str = "Deshabilitar denegaciones temporalmente. Reduce la seguridad; solo para investigación.";
pub const OPT_IGNORE_LABEL:        &str = "No hacer nada / Añadir a lista de ignorados";
pub const OPT_IGNORE_DESC:         &str = "Añadir esta entrada a la lista de ignorados (solo dentro de la herramienta).";

// ── Análisis de causa (estático) ──────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " La ruta no estándar requiere añadir una regla fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Ejecutar restorecon para restaurar el contexto predeterminado puede resolver esto.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Se necesita generar una política personalizada con audit2allow.";

// ── Nombres de Remedy ─────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Contexto de puerto";
pub const REMEDY_FILE_CONTEXT:  &str = "Contexto de archivo";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Política personalizada";

// ── Cadenas de formato ────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Denegaciones de acceso  [Hoy]  Sin resolver: {} / Total: {} ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" Módulos de política  {} módulos ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("Eliminar módulo de política '{}'.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("Módulo '{}' eliminado.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("Cargadas {} entradas AVC", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Comando fallido: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Bloqueado ({} segundos restantes)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Log  {} entradas  ↑↓:Desplazar  l:Cerrar ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Añadir contexto de puerto  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Asignar contexto ssh_port_t al puerto {} de {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Reparar con restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Cambiar fcontext + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Añadir regla para asignar {} a esta ruta y ejecutar restorecon automáticamente.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Habilitar Boolean (temporal)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Habilitar {} (se revierte al reiniciar).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Habilitar Boolean (permanente)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Habilitar {} permanentemente.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Establecer dominio como Permissive (solo investigación) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" A {} se le denegó {} en {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" El puerto {} no está definido en la política SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} necesita un contexto de puerto para operar en un puerto no estándar.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Se denegó el acceso de escritura a {}.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" La etiqueta en {} puede haber sido eliminada.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Habilitar el Boolean {} puede resolver esto.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" La operación {} del dominio {} no está permitida por la política.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("hace {}s", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("hace {}m", n) }
pub fn elapsed_hours(n: u64) -> String { format!("hace {}h", n) }
pub fn elapsed_days(n: u64)  -> String { format!("hace {}d", n) }
pub const LABEL_FIRST_SEEN: &str = "Primera aparición";
pub const LABEL_LAST_SEEN:  &str = "Última aparición";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Advertencia: La configuración regional puede no ser UTF-8 (LANG={}).\n\
         Establezca LANG=es_ES.UTF-8 si los caracteres no se muestran correctamente.",
        lang_val
    )
}

// ── Verificación de dependencias ──────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] No se encontraron algunos comandos necesarios:";
pub const WARN_MISSING_OPT_FTR: &str = "       Las funciones que usan los comandos anteriores no funcionarán.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] Comandos requeridos no encontrados. No se puede iniciar seadmin:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (paquete: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
Instale los paquetes anteriores y vuelva a intentarlo.\n\
  ej. (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  ej. (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── Salida de registro ────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin iniciado (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] No se pudo abrir el archivo de registro: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC cargado: {} entradas", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (sin ruta absoluta — restorecon/fcontext oculto)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] Error al cargar AVC: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] Comando exitoso";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] Autenticación fallida ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] Comando fallido:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] Modo SELinux: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow generado: {} líneas, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({} líneas de registro como entrada)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (autenticación en caché)", cmd) }

// ── Errores de comando ────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "Sin permiso para leer audit.log. Agréguese al grupo adm o configure sudo.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow falló: {}", stderr) }
