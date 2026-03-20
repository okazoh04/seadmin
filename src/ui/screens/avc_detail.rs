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
            let path = entry.target.trim_matches('"').to_string();
            let has_abs_path = path.starts_with('/');

            if has_abs_path {
                opts.push(RemOption {
                    key: 'A',
                    label: lang.opt_restorecon_label(&path),
                    command: vec!["restorecon".into(), "-Rv".into(), path.clone()],
                    description: lang.opt_restorecon_desc().to_string(),
                    needs_auth: true,
                    warning: false,
                });
            }

            let file_type = entry
                .tcontext
                .split(':')
                .nth(2)
                .unwrap_or("unlabeled_t")
                .to_string();

            if has_abs_path {
                opts.push(RemOption {
                    key: 'B',
                    label: lang.opt_fcontext_label(&file_type, &path),
                    command: vec![
                        "semanage".into(),
                        "fcontext".into(),
                        "-a".into(),
                        "-t".into(),
                        file_type.clone(),
                        format!("{}(.*)", path),
                    ],
                    description: lang.opt_fcontext_desc(&file_type),
                    needs_auth: true,
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
    lines.push(String::new());
    match &entry.remedy {
        crate::selinux::avc::Remedy::PortContext => {
            lines.push(lang.analysis_port_undefined(&entry.target));
            lines.push(lang.analysis_port_nonstandard(&entry.process));
        }
        crate::selinux::avc::Remedy::FileContext => {
            lines.push(lang.analysis_write_denied(&entry.target));
            lines.push(lang.analysis_fcontext_nonstandard().to_string());
        }
        crate::selinux::avc::Remedy::Restorecon => {
            lines.push(lang.analysis_label_stripped(&entry.target));
            lines.push(lang.analysis_restorecon_fix().to_string());
        }
        crate::selinux::avc::Remedy::Boolean(b) => {
            lines.push(lang.analysis_bool_enable(b));
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
