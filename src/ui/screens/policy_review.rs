use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, te: &str, scroll: usize) {
    let lines: Vec<Line> = te
        .lines()
        .map(|l| Line::from(Span::styled(l.to_string(), Style::default().fg(Color::Rgb(144, 238, 144)))))
        .collect();

    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" ポリシー内容確認（Enter:適用  Esc:キャンセル）")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .scroll((scroll as u16, 0));

    f.render_widget(para, area);
}
