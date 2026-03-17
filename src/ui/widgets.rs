/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use std::collections::VecDeque;

use crate::i18n::Lang;
use chrono::Local;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

/// ヘッダーバーを描画する
pub fn render_header(f: &mut Frame, area: Rect, selinux_mode: &str, hostname: &str, lang: &Lang) {
    let mode_color = if selinux_mode == "Enforcing" {
        Color::Green
    } else if selinux_mode == "Permissive" {
        Color::Yellow
    } else {
        Color::Red
    };

    let now = Local::now().format(lang.datetime_format()).to_string();

    let line = Line::from(vec![
        Span::styled(
            " seadmin v0.2",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("│ SELinux: "),
        Span::styled(
            selinux_mode,
            Style::default()
                .fg(mode_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" │ {} │ {} ", hostname, now)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Rgb(26, 26, 46)).fg(Color::White));

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}

/// フッターバーを描画する
pub fn render_footer(f: &mut Frame, area: Rect, hint: &str) {
    let para = Paragraph::new(Line::from(Span::styled(
        format!(" {}", hint),
        Style::default().fg(Color::DarkGray),
    )))
    .style(Style::default().bg(Color::Rgb(17, 17, 17)));
    f.render_widget(para, area);
}

/// ポップアップ用に中央の矩形を返す
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_w = r.width * percent_x / 100;
    let popup_h = r.height * percent_y / 100;
    let x = r.x + (r.width.saturating_sub(popup_w)) / 2;
    let y = r.y + (r.height.saturating_sub(popup_h)) / 2;
    Rect::new(x, y, popup_w, popup_h)
}

/// 背景をクリアしてポップアップ枠を描画する
pub fn render_popup_frame(f: &mut Frame, area: Rect, title: &str) {
    f.render_widget(Clear, area);
    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Rgb(13, 13, 13)).fg(Color::White));
    f.render_widget(block, area);
}

/// ログオーバーレイを描画する（新しいエントリが上）
pub fn render_log_overlay(f: &mut Frame, area: Rect, log: &VecDeque<String>, scroll: usize, lang: &Lang) {
    let popup = centered_rect(92, 85, area);
    f.render_widget(Clear, popup);

    let inner_height = popup.height.saturating_sub(2) as usize;
    let total = log.len();
    let lines: Vec<Line> = log
        .iter()
        .rev()
        .skip(scroll)
        .take(inner_height)
        .map(|entry| {
            let color = if entry.contains("[ERR]") || entry.contains("[WARN]") {
                Color::Red
            } else if entry.contains("[CMD]") {
                Color::Cyan
            } else if entry.contains("[OK]") {
                Color::Green
            } else {
                Color::Rgb(180, 180, 180)
            };
            Line::from(Span::styled(entry.clone(), Style::default().fg(color)))
        })
        .collect();

    let title = lang.log_overlay_title(total);
    let para = Paragraph::new(lines).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(para, popup);
}

/// ステータスメッセージバーを描画する（画面下部 1行）
pub fn render_status(f: &mut Frame, area: Rect, msg: &str, is_error: bool) {
    let color = if is_error { Color::Red } else { Color::Green };
    let para = Paragraph::new(Span::styled(
        format!(" {}", msg),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ));
    f.render_widget(para, area);
}
