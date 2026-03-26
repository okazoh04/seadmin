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

/// カスタムモジュールと判定する優先度の閾値（seadmin は -X 300 で適用する）
const CUSTOM_PRIORITY_THRESHOLD: u16 = 300;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let lang = &app.lang;
    let all_modules = &app.module_list;
    let show_all = app.module_show_all;

    // 表示対象を絞り込む
    let displayed: Vec<_> = if show_all {
        all_modules.iter().collect()
    } else {
        all_modules
            .iter()
            .filter(|m| m.priority >= CUSTOM_PRIORITY_THRESHOLD)
            .collect()
    };

    let custom_count = all_modules
        .iter()
        .filter(|m| m.priority >= CUSTOM_PRIORITY_THRESHOLD)
        .count();
    let total_count = all_modules.len();

    let title = lang.module_list_title(custom_count, total_count, show_all);

    // 表示可能な行数を計算（上下ボーダー各1行 + ヘッダー行1行 = 3行のオーバーヘッド）
    let visible_rows = (area.height as usize).saturating_sub(3);

    // カーソルが表示範囲に入るようにオフセットを計算
    let offset = if app.module_cursor >= visible_rows {
        app.module_cursor - visible_rows + 1
    } else {
        0
    };

    // ヘッダー行
    let header_line = Line::from(vec![
        Span::styled(
            format!(" {:<9}", lang.col_priority()),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            lang.col_module_name(),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ])
    .style(Style::default().bg(Color::Rgb(26, 26, 46)));

    // データ行
    let mut lines: Vec<Line> = vec![header_line];
    for (i, m) in displayed.iter().enumerate().skip(offset).take(visible_rows) {
        let selected = i == app.module_cursor;
        let is_custom = m.priority >= CUSTOM_PRIORITY_THRESHOLD;

        let row_style = if selected {
            Style::default().bg(Color::Rgb(26, 82, 118)).fg(Color::White)
        } else if is_custom {
            Style::default().fg(Color::Rgb(255, 215, 100))
        } else {
            Style::default().fg(Color::Rgb(120, 120, 120))
        };

        let (prefix_span, priority_str) = if selected {
            (
                Span::styled("▶ ", Style::default().fg(Color::White)),
                format!("{:<7} ", m.priority),
            )
        } else if is_custom {
            (
                Span::styled("* ", Style::default().fg(Color::Rgb(255, 180, 50))),
                format!("{:<7} ", m.priority),
            )
        } else {
            (
                Span::raw("  "),
                format!("{:<7} ", m.priority),
            )
        };

        let line = Line::from(vec![
            prefix_span,
            Span::styled(priority_str, row_style),
            Span::styled(m.name.clone(), row_style),
        ]);
        lines.push(line);
    }

    let para = Paragraph::new(lines).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 149, 237))),
    );

    f.render_widget(para, area);
}
