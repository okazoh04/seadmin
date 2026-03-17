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
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

pub fn render(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let lang = &app.lang;
    let entries = app.filtered_avc();
    let unresolved = entries.iter().filter(|e| !e.resolved).count();
    let total = entries.len();

    let title = lang.avc_list_title(unresolved, total);

    let header = Row::new(vec![
        Cell::from(" # ").style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_occurred()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_process()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_action()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_target()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_count()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_remedy()).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ])
    .style(Style::default().bg(Color::Rgb(26, 26, 46)))
    .height(1);

    let rows: Vec<Row> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let selected = i == app.avc_cursor;
            let style = if selected {
                Style::default()
                    .bg(Color::Rgb(26, 82, 118))
                    .fg(Color::White)
            } else if e.resolved {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Rgb(200, 200, 200))
            };

            let prefix = if selected { "▶" } else { " " };

            Row::new(vec![
                Cell::from(format!("{}{}", prefix, e.id)),
                Cell::from(e.elapsed_str(lang)),
                Cell::from(e.process.clone()),
                Cell::from(e.perm.clone()),
                Cell::from(truncate(&e.target, 24)),
                Cell::from(e.count.to_string()),
                Cell::from(e.remedy.display_str(lang)),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(9),
            Constraint::Length(12),
            Constraint::Length(14),
            Constraint::Length(26),
            Constraint::Length(6),
            Constraint::Min(14),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(100, 100, 140))),
    )
    .row_highlight_style(Style::default());

    let mut state = TableState::default().with_selected(Some(app.avc_cursor));
    f.render_stateful_widget(table, area, &mut state);

    // フィルタ入力中の表示
    if app.avc_filter_active || !app.avc_filter.is_empty() {
        let filter_text = format!("{}{}▌", lang.filter_label(), app.avc_filter);
        let filter_line = Line::from(vec![Span::styled(
            filter_text,
            Style::default().fg(Color::Yellow),
        )]);
        use ratatui::layout::Rect;
        let filter_area = Rect::new(area.x + 1, area.bottom().saturating_sub(2), area.width.saturating_sub(2), 1);
        f.render_widget(ratatui::widgets::Paragraph::new(filter_line), filter_area);
    }
}

fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max {
        format!("{}…", &chars[..max - 1].iter().collect::<String>())
    } else {
        s.to_string()
    }
}
