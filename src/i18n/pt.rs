/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── Dicas no rodapé ───────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:Mover  Enter:Detalhes  /:Filtrar  r:Recarregar  l:Log  q:Sair";
pub const HINT_AVC_DETAIL:   &str = "A-F:Selecionar  Esc/←:Voltar  Enter:Confirmar";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:Rolar  Enter:Aplicar  Esc:Cancelar";
pub const HINT_AUTH:         &str = "Enter:Executar  Esc:Cancelar";

// ── Cabeçalhos da tabela ──────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "Quando";
pub const COL_PROCESS:  &str = "Processo";
pub const COL_ACTION:   &str = "Ação";
pub const COL_TARGET:   &str = "Destino";
pub const COL_COUNT:    &str = "Contagem";
pub const COL_REMEDY:   &str = "Solução";

// ── Status / mensagens ────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ Carregando log AVC...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux está desativado. Negações de acesso não serão registradas.";
pub const NO_AVC:           &str = "Nenhuma negação de acesso";
pub const OP_COMPLETE:      &str = "Operação concluída";
pub const IGNORED:          &str = "Adicionado à lista de ignorados";
pub const FILTER_LABEL:     &str = "/Filtrar: ";

// ── Janela de autenticação ────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 Autenticação de administrador";
pub const AUTH_CMD_LABEL:  &str = "  Comando:";
pub const AUTH_PW_LABEL:   &str = "  Senha:";
pub const AUTH_CANCEL_BTN: &str = "[ Cancelar (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ Executar (Enter) ]";
pub const PW_WRONG:        &str = "Senha incorreta";

// ── Blocos da tela de detalhes ────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " Análise de causa ";
pub const BLOCK_OPTIONS:  &str = " Opções de solução ";
pub const BLOCK_RAW_LOG:  &str = " Log bruto (referência)";

// ── Revisão de política ───────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " Revisão de política (Enter:Aplicar  Esc:Cancelar)";
pub const POLICY_APPLY_DESC:   &str = "Aplicar o módulo de política gerado ao sistema.";

// ── Opções de solução (estáticas) ─────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "Restaurar o contexto de arquivo padrão (reparar rótulos perdidos).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "Gerar e aplicar módulo de política personalizado (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "Gerar automaticamente uma política com audit2allow. Revise antes de aplicar.";
pub const OPT_PERMISSIVE_DESC:     &str = "Desativar negações temporariamente. Reduz a segurança; use apenas para investigação.";
pub const OPT_IGNORE_LABEL:        &str = "Não fazer nada / Adicionar à lista de ignorados";
pub const OPT_IGNORE_DESC:         &str = "Adicionar esta entrada à lista de ignorados (somente dentro da ferramenta).";

// ── Análise de causa (estática) ───────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " Caminho não padrão requer adição de uma regra fcontext.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " Executar restorecon para restaurar o contexto padrão pode resolver isso.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " Uma política personalizada precisa ser gerada com audit2allow.";

// ── Nomes de Remedy ───────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "Contexto de porta";
pub const REMEDY_FILE_CONTEXT:  &str = "Contexto de arquivo";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "Política personalizada";

// ── Strings de formato ────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" Negações de acesso  [Hoje]  Não resolvidas: {} / Total: {} ", unresolved, total)
}
pub fn avc_loaded(count: usize) -> String {
    format!("{} entradas AVC carregadas", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("Falha no comando: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  Bloqueado ({} segundos restantes)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" Log  {} entradas  ↑↓:Rolar  l:Fechar ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("Adicionar contexto de porta  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("Atribuir contexto ssh_port_t à porta {} de {}.", target, proto)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("Reparar com restorecon  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("Alterar fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("Adicionar regra para atribuir {} a este caminho. Execute restorecon após aplicar.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Habilitar Boolean (temporário)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("Habilitar {} (reverte após reinicialização).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Habilitar Boolean (permanente)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("Habilitar {} permanentemente.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("Definir domínio como Permissive (apenas investigação) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {} teve {} negado em {}.", process, perm, target)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" A porta {} não está definida na política SELinux.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {} precisa de um contexto de porta para operar em uma porta não padrão.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" Acesso de escrita a {} foi negado.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" O rótulo em {} pode ter sido removido.", target)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" Habilitar o Boolean {} pode resolver isso.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" A operação {} do domínio {} não é permitida pela política.", perm, domain)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("há {}s", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("há {}m", n) }
pub fn elapsed_hours(n: u64) -> String { format!("há {}h", n) }
pub fn elapsed_days(n: u64)  -> String { format!("há {}d", n) }
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "Aviso: A localidade pode não ser UTF-8 (LANG={}).\n\
         Defina LANG=pt_BR.UTF-8 se os caracteres não aparecerem corretamente.",
        lang_val
    )
}
