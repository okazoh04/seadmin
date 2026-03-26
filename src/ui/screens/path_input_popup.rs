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
use crate::ui::widgets::{centered_rect, render_popup_frame};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let lang = &app.lang;
    let popup = centered_rect(65, 45, area);
    render_popup_frame(f, popup, lang.path_input_title());

    let inner = Rect::new(
        popup.x + 1,
        popup.y + 1,
        popup.width.saturating_sub(2),
        popup.height.saturating_sub(2),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // 余白
            Constraint::Length(2), // 説明文
            Constraint::Length(1), // 余白
            Constraint::Length(1), // 入力欄
            Constraint::Length(1), // 余白
            Constraint::Min(1),    // 余白
            Constraint::Length(1), // ボタン行
        ])
        .split(inner);

    // 説明文
    f.render_widget(
        Paragraph::new(Span::styled(
            lang.path_input_prompt(),
            Style::default().fg(Color::Rgb(200, 200, 200)),
        )),
        chunks[1],
    );

    // 入力欄
    let input_display = format!("  {}_", app.path_input_buf);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            input_display,
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Rgb(26, 26, 26)),
        ))),
        chunks[3],
    );

    // ボタン行
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("         "),
            Span::styled(
                lang.path_input_hint(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        chunks[6],
    );
}
