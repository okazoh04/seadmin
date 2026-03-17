use crate::i18n::Lang;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, te: &str, scroll: usize, lang: &Lang) {
    // .scroll() を使わず手動スライス — Paragraph::scroll はブロック内の
    // 未描画セルを上書きしないため残像が発生する場合がある
    // ratatui はタブ文字を1セル扱いするため、スペースに展開してから描画する
    let lines: Vec<Line> = te
        .lines()
        .skip(scroll)
        .map(|l| {
            let expanded = l.replace('\t', "    ");
            Line::from(Span::styled(expanded, Style::default().fg(Color::Rgb(144, 238, 144))))
        })
        .collect();

    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .title(lang.policy_review_title())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(para, area);
}
