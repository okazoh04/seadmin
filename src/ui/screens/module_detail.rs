/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use crate::ui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &App, name: &str, priority: u16, cil: &str) {
    let lang = &app.lang;

    // 明示的な Rect 計算（Layout::split を使わない）
    let header_h = 3u16.min(area.height);
    let header_area = Rect::new(area.x, area.y, area.width, header_h);
    let cil_h = area.height.saturating_sub(header_h);
    let cil_area = Rect::new(area.x, area.y + header_h, area.width, cil_h);

    // ── ヘッダー（モジュール名・優先度） ─────────────────────────────────────
    let header_text = vec![Line::from(vec![
        Span::styled(
            format!("  {} ", name),
            Style::default()
                .fg(Color::Rgb(255, 215, 100))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  priority: {}  ", priority),
            Style::default().fg(Color::Rgb(150, 150, 200)),
        ),
    ])];

    let header = Paragraph::new(header_text).block(
        Block::default()
            .title(lang.module_detail_title())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 149, 237))),
    );
    f.render_widget(header, header_area);

    // ── CIL ルール表示（スクロール対応） ─────────────────────────────────────
    let lines: Vec<Line> = cil
        .lines()
        .skip(app.module_detail_scroll)
        .map(|l| {
            let expanded = l.replace('\t', "    ");
            if expanded.trim_start().starts_with("(allow") {
                Line::from(Span::styled(
                    expanded,
                    Style::default().fg(Color::Rgb(144, 238, 144)),
                ))
            } else if expanded.trim_start().starts_with("(typeattributeset") {
                Line::from(Span::styled(
                    expanded,
                    Style::default().fg(Color::Rgb(150, 150, 200)),
                ))
            } else {
                Line::from(Span::styled(
                    expanded,
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ))
            }
        })
        .collect();

    let cil_block = Paragraph::new(lines).block(
        Block::default()
            .title(lang.module_detail_cil_title())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(70, 130, 70))),
    );
    f.render_widget(cil_block, cil_area);
}
