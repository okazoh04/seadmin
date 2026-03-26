/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use crate::i18n::Lang;
use crate::selinux::avc::{AvcEntry, Remedy};
use crate::ui::app::{AuthContext, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

/// 対処オプション
#[derive(Debug, Clone)]
pub struct RemOption {
    pub key: char,
    pub label: String,
    pub command: Vec<String>,
    pub description: String,
    pub needs_auth: bool,
    pub warning: bool,
}

/// AvcEntry から対処オプション一覧を生成する
pub fn build_options(entry: &AvcEntry, lang: &Lang) -> Vec<RemOption> {
    let mut opts = Vec::new();

    match &entry.remedy {
        Remedy::PortContext => {
            // entry.target が dest= の値（数字）か tcontext のフォールバックかを判定
            // tcontext 形式 (user:role:type:level) の場合はパースに失敗するため
            // raw_lines の dest= フィールドも追加で検索する
            let port_opt = entry
                .target
                .parse::<u16>()
                .ok()
                .or_else(|| {
                    entry.raw_lines.iter().find_map(|line| {
                        line.split_whitespace()
                            .find(|s| s.starts_with("dest="))
                            .and_then(|s| s["dest=".len()..].parse::<u16>().ok())
                    })
                })
                .map(|p| p.to_string());

            if let Some(port) = port_opt {
                let proto = if entry.tclass.contains("udp") { "udp" } else { "tcp" };
                let setype = entry.tcontext.split(':').nth(2).unwrap_or("port_t");
                opts.push(RemOption {
                    key: 'A',
                    label: lang.opt_port_label(proto, &port),
                    command: vec![
                        "semanage".into(),
                        "port".into(),
                        "-a".into(),
                        "-t".into(),
                        setype.into(),
                        "-p".into(),
                        proto.into(),
                        port,
                    ],
                    description: lang.opt_port_desc(proto, &entry.target),
                    needs_auth: true,
                    warning: false,
                });
            }
        }
        Remedy::FileContext | Remedy::Restorecon => {
            // override_path が指定されていればそれを優先、なければ target を使用
            let raw_path = entry
                .override_path
                .as_deref()
                .unwrap_or(&entry.target)
                .trim_matches('"')
                .to_string();
            let has_abs_path = raw_path.starts_with('/');

            if has_abs_path {
                opts.push(RemOption {
                    key: 'A',
                    label: lang.opt_restorecon_label(&raw_path),
                    command: vec!["restorecon".into(), "-Rv".into(), raw_path.clone()],
                    description: lang.opt_restorecon_desc().to_string(),
                    needs_auth: true,
                    warning: false,
                });
            }

            // tcontext の型は「現在の誤ったラベル」なので fcontext には使わない。
            // パスと scontext から適切な型を推測する。
            let (file_type, _certain) = suggest_fcontext_type(&raw_path, &entry.scontext);

            if has_abs_path {
                // semanage fcontext + restorecon の 2 ステップを sh -c で一括実行
                let sh_cmd = format!(
                    "semanage fcontext -a -t '{}' '{}(/.*)?'; restorecon -Rv '{}'",
                    file_type, raw_path, raw_path
                );
                opts.push(RemOption {
                    key: 'B',
                    label: lang.opt_fcontext_label(file_type, &raw_path),
                    command: vec!["sh".into(), "-c".into(), sh_cmd],
                    description: lang.opt_fcontext_desc(file_type),
                    needs_auth: true,
                    warning: false,
                });
            }

            // パスが不明な場合は P オプションを表示して最善策への誘導を促す
            if !has_abs_path {
                opts.push(RemOption {
                    key: 'P',
                    label: lang.opt_path_input_label().to_string(),
                    command: vec![],
                    description: lang.opt_path_input_desc().to_string(),
                    needs_auth: false,
                    warning: false,
                });
            }
        }
        Remedy::Boolean(bool_name) => {
            opts.push(RemOption {
                key: 'A',
                label: lang.opt_bool_temp_label(bool_name),
                command: vec!["setsebool".into(), bool_name.clone(), "on".into()],
                description: lang.opt_bool_temp_desc(bool_name),
                needs_auth: true,
                warning: false,
            });
            opts.push(RemOption {
                key: 'B',
                label: lang.opt_bool_perm_label(bool_name),
                command: vec!["setsebool".into(), "-P".into(), bool_name.clone(), "on".into()],
                description: lang.opt_bool_perm_desc(bool_name),
                needs_auth: true,
                warning: false,
            });
        }
        _ => {}
    }

    // 共通オプション
    opts.push(RemOption {
        key: 'C',
        label: lang.opt_custom_policy_label().to_string(),
        command: vec![],
        description: lang.opt_custom_policy_desc().to_string(),
        needs_auth: true,
        warning: false,
    });
    let domain = entry.scontext.split(':').nth(2).unwrap_or("domain_t");
    opts.push(RemOption {
        key: 'D',
        label: lang.opt_permissive_label(domain),
        command: vec![
            "semanage".into(),
            "permissive".into(),
            "-a".into(),
            domain.to_string(),
        ],
        description: lang.opt_permissive_desc().to_string(),
        needs_auth: true,
        warning: true,
    });
    opts.push(RemOption {
        key: 'F',
        label: lang.opt_ignore_label().to_string(),
        command: vec![],
        description: lang.opt_ignore_desc().to_string(),
        needs_auth: false,
        warning: false,
    });

    opts
}

/// FileContext remedy で semanage fcontext に使う型を推測する。
/// tcontext の型（現在の間違ったラベル）は使わず、パスと scontext から判断する。
/// 戻り値: (推奨型, 確実かどうか)
fn suggest_fcontext_type(path: &str, scontext: &str) -> (&'static str, bool) {
    let stype = scontext.split(':').nth(2).unwrap_or("");
    let p = path.to_ascii_lowercase();

    // ログファイル系（パスに log が含まれる、またはアクセス元が logrotate）
    if p.contains("/log") || p.ends_with(".log") || stype.contains("logrotate") {
        if stype.contains("httpd") || stype.contains("nginx") || stype.contains("php") {
            return ("httpd_log_t", false);
        }
        return ("var_log_t", false);
    }
    // httpd コンテンツ
    if stype.contains("httpd_t") || stype.contains("nginx_t") {
        return ("httpd_sys_content_t", false);
    }
    // MySQL / MariaDB
    if stype.contains("mysqld_t") || stype.contains("mariadb_t") {
        return ("mysqld_db_t", false);
    }
    // その他：restorecon が先なので型不明を示す
    ("var_t", false)
}

pub fn render(f: &mut Frame, area: Rect, entry: &AvcEntry, cursor: usize, options: &[RemOption], lang: &Lang) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(8),
            Constraint::Length(5),
        ])
        .split(area);

    // --- 原因分析ブロック ---
    let analysis = build_analysis_text(entry, lang);
    let analysis_para = Paragraph::new(
        analysis
            .iter()
            .map(|s| Line::from(s.as_str()))
            .collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .title(lang.block_analysis())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 100, 140))),
    )
    .wrap(Wrap { trim: false });
    f.render_widget(analysis_para, chunks[0]);

    // --- 対処オプションブロック ---
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let prefix = if i == cursor { "▶ " } else { "  " };
            let style = if i == cursor {
                Style::default()
                    .bg(Color::Rgb(26, 82, 118))
                    .fg(Color::White)
            } else if opt.warning {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Rgb(200, 200, 200))
            };
            let label = format!("[{}] {}", opt.key, opt.label);
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(label, style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default().with_selected(Some(cursor));
    let list = List::new(items).block(
        Block::default()
            .title(lang.block_options())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 100, 140))),
    );
    f.render_stateful_widget(list, chunks[1], &mut list_state);

    // --- 生ログブロック ---
    let raw_text: Vec<Line> = entry
        .raw_lines
        .iter()
        .take(3)
        .map(|l| {
            Line::from(Span::styled(
                l.clone(),
                Style::default().fg(Color::Rgb(100, 200, 100)),
            ))
        })
        .collect();
    let raw_para = Paragraph::new(raw_text)
        .block(
            Block::default()
                .title(lang.block_raw_log())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 100, 140))),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(raw_para, chunks[2]);
}

fn build_analysis_text(entry: &AvcEntry, lang: &Lang) -> Vec<String> {
    let mut lines = Vec::new();
    let domain = entry.scontext.split(':').nth(2).unwrap_or(&entry.scontext);
    lines.push(lang.analysis_denied(&entry.process, &entry.target, &entry.perm));
    // syscall / errno（SYSCALL レコードがある場合のみ表示）
    if entry.syscall_name.is_some() || entry.errno_name.is_some() {
        let mut parts = Vec::new();
        if let Some(sc) = &entry.syscall_name {
            parts.push(format!("{}: {}", lang.label_syscall(), sc));
        }
        if let Some(en) = &entry.errno_name {
            parts.push(format!("{}: {}", lang.label_errno(), en));
        }
        lines.push(format!(" {}", parts.join("  ")));
    }
    // first_seen / last_seen（複数回発生している場合のみ両方表示）
    if entry.count > 1 {
        let fmt = lang.datetime_format();
        let first = entry.first_seen.format(fmt).to_string();
        let last  = entry.last_seen.format(fmt).to_string();
        lines.push(format!(" {}: {}  {}: {}", lang.label_first_seen(), first, lang.label_last_seen(), last));
    }
    lines.push(String::new());
    match &entry.remedy {
        crate::selinux::avc::Remedy::PortContext => {
            lines.push(lang.analysis_port_undefined(&entry.target));
            lines.push(lang.analysis_port_nonstandard(&entry.process));
        }
        crate::selinux::avc::Remedy::FileContext => {
            let eff_path = entry.override_path.as_deref().unwrap_or(&entry.target);
            lines.push(lang.analysis_write_denied(eff_path));
            lines.push(lang.analysis_fcontext_nonstandard().to_string());
            if !eff_path.starts_with('/') {
                lines.push(lang.analysis_path_unknown_hint().to_string());
            }
            // tclass=dir かつ絶対パスが取得できている場合は ls -dZ での確認を促す
            if entry.tclass == "dir" && eff_path.starts_with('/') {
                lines.push(lang.analysis_dir_label_check(eff_path));
            }
        }
        crate::selinux::avc::Remedy::Restorecon => {
            let eff_path = entry.override_path.as_deref().unwrap_or(&entry.target);
            lines.push(lang.analysis_label_stripped(eff_path));
            lines.push(lang.analysis_restorecon_fix().to_string());
            if !eff_path.starts_with('/') {
                lines.push(lang.analysis_path_unknown_hint().to_string());
            }
        }
        crate::selinux::avc::Remedy::Boolean(b) => {
            lines.push(lang.analysis_bool_enable(b));
            if let Some(desc) = &entry.bool_description {
                lines.push(format!(" ℹ {}", desc));
            }
        }
        _ => {
            lines.push(lang.analysis_domain_denied(domain, &entry.perm));
            lines.push(lang.analysis_custompolicy_fix().to_string());
        }
    }
    lines
}

/// キー入力からオプションを選択して AuthContext を返す（認証が必要な場合）
pub fn select_option(entry: &AvcEntry, options: &[RemOption], cursor: usize) -> Option<AuthContext> {
    let opt = options.get(cursor)?;
    if !opt.needs_auth || opt.command.is_empty() {
        return None;
    }
    Some(AuthContext {
        command: opt.command.clone(),
        description: opt.description.clone(),
        prev_screen: Box::new(Screen::AvcDetail(entry.id)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;
    use chrono::Local;

    #[test]
    fn test_build_options_port() {
        let lang = Lang::English;
        let entry = AvcEntry {
            id: 1, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "nginx".into(), perm: "name_bind".into(), tclass: "tcp_socket".into(),
            scontext: "system_u:system_r:httpd_t:s0".into(),
            tcontext: "system_u:object_r:http_port_t:s0".into(),
            target: "8080".into(),
            raw_lines: vec!["type=AVC ... dest=8080 ...".into()],
            remedy: Remedy::PortContext,
            resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };

        let opts = build_options(&entry, &lang);
        let port_opt = opts.iter().find(|o| o.key == 'A').expect("Option A missing");
        assert_eq!(port_opt.command, vec!["semanage", "port", "-a", "-t", "http_port_t", "-p", "tcp", "8080"]);
    }

    #[test]
    fn test_build_options_file_context() {
        let lang = Lang::English;
        let entry = AvcEntry {
            id: 1, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "httpd".into(), perm: "write".into(), tclass: "file".into(),
            scontext: "system_u:system_r:httpd_t:s0".into(),
            tcontext: "system_u:object_r:default_t:s0".into(),
            target: "/var/www/html/upload".into(),
            raw_lines: vec![],
            remedy: Remedy::FileContext,
            resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };

        let opts = build_options(&entry, &lang);
        
        // Restorecon option
        let res_opt = opts.iter().find(|o| o.key == 'A').expect("Option A missing");
        assert_eq!(res_opt.command, vec!["restorecon", "-Rv", "/var/www/html/upload"]);

        // Fcontext option
        let fc_opt = opts.iter().find(|o| o.key == 'B').expect("Option B missing");
        assert_eq!(fc_opt.command[0], "sh");
        assert_eq!(fc_opt.command[1], "-c");
        assert!(fc_opt.command[2].contains("semanage fcontext -a -t 'httpd_sys_content_t' '/var/www/html/upload(/.*)?'"));
        assert!(fc_opt.command[2].contains("restorecon -Rv '/var/www/html/upload'"));
    }

    #[test]
    fn test_build_options_boolean() {
        let lang = Lang::English;
        let entry = AvcEntry {
            id: 1, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "httpd".into(), perm: "name_connect".into(), tclass: "tcp_socket".into(),
            scontext: "system_u:system_r:httpd_t:s0".into(),
            tcontext: "system_u:object_r:http_port_t:s0".into(),
            target: "80".into(),
            raw_lines: vec![],
            remedy: Remedy::Boolean("httpd_can_network_connect".into()),
            resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };

        let opts = build_options(&entry, &lang);
        
        // Temporary boolean
        let temp_opt = opts.iter().find(|o| o.key == 'A').expect("Option A missing");
        assert_eq!(temp_opt.command, vec!["setsebool", "httpd_can_network_connect", "on"]);

        // Persistent boolean
        let perm_opt = opts.iter().find(|o| o.key == 'B').expect("Option B missing");
        assert_eq!(perm_opt.command, vec!["setsebool", "-P", "httpd_can_network_connect", "on"]);
    }
}
