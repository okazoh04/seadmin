use crate::ui::app::{App, AuthContext};
use crate::ui::widgets::{centered_rect, render_popup_frame};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &App, ctx: &AuthContext) {
    let popup = centered_rect(65, 50, area);
    render_popup_frame(f, popup, "🔒 管理者認証");

    // ポップアップ内部のレイアウト
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
            Constraint::Length(1), // "実行コマンド："
            Constraint::Length(2), // コマンド本文
            Constraint::Length(1), // 余白
            Constraint::Length(1), // "パスワード："
            Constraint::Length(1), // パスワード入力欄
            Constraint::Length(1), // 余白
            Constraint::Length(1), // エラーメッセージ
            Constraint::Min(1),    // 余白
            Constraint::Length(1), // ボタン
        ])
        .split(inner);

    // "実行コマンド："ラベル
    f.render_widget(
        Paragraph::new(Span::raw("  実行コマンド：")),
        chunks[1],
    );

    // コマンド本文
    let cmd_str = ctx.command.join(" ");
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("  sudo {}", cmd_str),
            Style::default().fg(Color::Green),
        ))),
        chunks[2],
    );

    // パスワードラベル
    f.render_widget(
        Paragraph::new(Span::raw("  パスワード：")),
        chunks[4],
    );

    // パスワード入力欄（マスク表示）
    let dot_count = app.password_buf.len();
    let pw_display = format!("  {}_", "•".repeat(dot_count));
    let pw_style = if app.auth_state.is_locked() || app.auth_error.is_some() {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Rgb(26, 26, 26))
    } else {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Rgb(26, 26, 26))
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(pw_display, pw_style))),
        chunks[5],
    );

    // エラーメッセージ / ロックアウト
    if app.auth_state.is_locked() {
        let secs = app.auth_state.lock_remaining_secs();
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                format!("  ロックアウト中（{}秒後に解除）", secs),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))),
            chunks[7],
        );
    } else if let Some(err) = &app.auth_error {
        let fail_count = app.auth_state.fail_count;
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                format!("  {} ({}/3)", err, fail_count),
                Style::default().fg(Color::Red),
            ))),
            chunks[7],
        );
    }

    // ボタン行
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("         "),
            Span::styled(
                "[ キャンセル（Esc） ]",
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw("  "),
            Span::styled(
                "[ 実行（Enter） ]",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        chunks[9],
    );
}
