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

pub fn render(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let lang = &app.lang;
    let modules = &app.module_list;

    let title = lang.module_list_title(modules.len());

    let header = Row::new(vec![
        Cell::from(lang.col_priority())
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Cell::from(lang.col_module_name())
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ])
    .style(Style::default().bg(Color::Rgb(26, 26, 46)))
    .height(1);

    let rows: Vec<Row> = modules
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let selected = i == app.module_cursor;
            let style = if selected {
                Style::default()
                    .bg(Color::Rgb(26, 82, 118))
                    .fg(Color::White)
            } else {
                Style::default().fg(Color::Rgb(200, 200, 200))
            };

            let priority_cell = if selected {
                Cell::from(Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(Color::White)),
                    Span::raw(m.priority.to_string()),
                ]))
            } else {
                Cell::from(format!("  {}", m.priority))
            };

            Row::new(vec![
                priority_cell,
                Cell::from(m.name.clone()),
            ])
            .style(style)
            .height(1)
        })
        .collect();

    let widths = [Constraint::Length(10), Constraint::Min(20)];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 149, 237))),
        )
        .row_highlight_style(Style::default().bg(Color::Rgb(26, 82, 118)));

    let mut state = TableState::default();
    if !modules.is_empty() {
        state.select(Some(app.module_cursor));
    }

    f.render_stateful_widget(table, area, &mut state);
}
