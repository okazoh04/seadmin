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
pub fn build_options(entry: &AvcEntry) -> Vec<RemOption> {
    let mut opts = Vec::new();

    match &entry.remedy {
        Remedy::PortContext => {
            // ポート番号を target から推測（単純なケース）
            let port = entry
                .target
                .split(':')
                .last()
                .and_then(|s| s.parse::<u16>().ok())
                .map(|p| p.to_string())
                .unwrap_or_else(|| "????".to_string());
            let proto = if entry.tclass.contains("udp") { "udp" } else { "tcp" };
            opts.push(RemOption {
                key: 'A',
                label: format!(
                    "ポートコンテキストを追加  semanage port -a -t ssh_port_t -p {} {}",
                    proto, port
                ),
                command: vec![
                    "semanage".into(),
                    "port".into(),
                    "-a".into(),
                    "-t".into(),
                    "ssh_port_t".into(),
                    "-p".into(),
                    proto.into(),
                    port,
                ],
                description: format!(
                    "{} の {} ポートに ssh_port_t コンテキストを付与します。",
                    proto, entry.target
                ),
                needs_auth: true,
                warning: false,
            });
        }
        Remedy::FileContext | Remedy::Restorecon => {
            let path = entry.target.trim_matches('"').to_string();
            let has_abs_path = path.starts_with('/');

            if has_abs_path {
                opts.push(RemOption {
                    key: 'A',
                    label: format!("restorecon で修復  restorecon -Rv {}", path),
                    command: vec!["restorecon".into(), "-Rv".into(), path.clone()],
                    description: "デフォルトのファイルコンテキストに戻します（ラベル剥がれの修復）。".to_string(),
                    needs_auth: true,
                    warning: false,
                });
            }

            // tcontext からファイル型を取得（例: system_u:object_r:httpd_sys_content_t:s0 → httpd_sys_content_t）
            let file_type = entry
                .tcontext
                .split(':')
                .nth(2)
                .unwrap_or("unlabeled_t")
                .to_string();

            if has_abs_path {
                opts.push(RemOption {
                    key: 'B',
                    label: format!(
                        "fcontext を変更  semanage fcontext -a -t {} {}(.*)",
                        file_type, path
                    ),
                    command: vec![
                        "semanage".into(),
                        "fcontext".into(),
                        "-a".into(),
                        "-t".into(),
                        file_type.clone(),
                        format!("{}(.*)", path),
                    ],
                    description: format!(
                        "このパスに {} を付与するルールを追加します。適用後 restorecon も実行してください。",
                        file_type
                    ),
                    needs_auth: true,
                    warning: false,
                });
            }
        }
        Remedy::Boolean(bool_name) => {
            opts.push(RemOption {
                key: 'A',
                label: format!("Boolean を有効化（一時）  setsebool {} on", bool_name),
                command: vec!["setsebool".into(), bool_name.clone(), "on".into()],
                description: format!("{} を有効にします（再起動で元に戻ります）。", bool_name),
                needs_auth: true,
                warning: false,
            });
            opts.push(RemOption {
                key: 'B',
                label: format!("Boolean を有効化（永続）  setsebool -P {} on", bool_name),
                command: vec!["setsebool".into(), "-P".into(), bool_name.clone(), "on".into()],
                description: format!("{} を永続的に有効にします。", bool_name),
                needs_auth: true,
                warning: false,
            });
        }
        _ => {}
    }

    // 共通オプション
    opts.push(RemOption {
        key: 'C',
        label: "カスタムポリシーモジュールを生成・適用（audit2allow）".to_string(),
        command: vec![], // audit2allow は別フローで処理
        description: "audit2allow でポリシーを自動生成します。内容をレビューしてから適用できます。".to_string(),
        needs_auth: true,
        warning: false,
    });
    opts.push(RemOption {
        key: 'D',
        label: format!("このドメインを Permissive に設定（調査用）⚠  semanage permissive -a {}", entry.scontext.split(':').nth(2).unwrap_or("domain_t")),
        command: vec![
            "semanage".into(),
            "permissive".into(),
            "-a".into(),
            entry.scontext.split(':').nth(2).unwrap_or("domain_t").to_string(),
        ],
        description: "拒否を一時的に無効化します。セキュリティが低下するため調査目的に限定してください。".to_string(),
        needs_auth: true,
        warning: true,
    });
    opts.push(RemOption {
        key: 'F',
        label: "何もしない / 無視リストに追加".to_string(),
        command: vec![],
        description: "このエントリを無視リストに追加します（ツール内のみ）。".to_string(),
        needs_auth: false,
        warning: false,
    });

    opts
}

pub fn render(f: &mut Frame, area: Rect, entry: &AvcEntry, cursor: usize, options: &[RemOption]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // 原因分析
            Constraint::Min(8),     // 対処オプション
            Constraint::Length(5),  // 生ログ
        ])
        .split(area);

    // --- 原因分析ブロック ---
    let analysis = build_analysis_text(entry);
    let analysis_para = Paragraph::new(
        analysis
            .iter()
            .map(|s| Line::from(s.as_str()))
            .collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .title(" 原因分析 ")
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
            .title(" 対処オプション ")
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
                .title(" 生ログ（参照用）")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 100, 140))),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(raw_para, chunks[2]);
}

fn build_analysis_text(entry: &AvcEntry) -> Vec<String> {
    let mut lines = Vec::new();
    let domain = entry.scontext.split(':').nth(2).unwrap_or(&entry.scontext);
    lines.push(format!(
        " {} が {} への {} を拒否されました。",
        entry.process, entry.target, entry.perm
    ));
    lines.push(String::new());
    match &entry.remedy {
        crate::selinux::avc::Remedy::PortContext => {
            lines.push(format!(
                " ポート {} は SELinux ポリシー上で未定義です。",
                entry.target
            ));
            lines.push(format!(
                " {} を非標準ポートで動作させるにはポートコンテキストの追加が必要です。",
                entry.process
            ));
        }
        crate::selinux::avc::Remedy::FileContext => {
            lines.push(format!(
                " {} への書き込みが拒否されました。",
                entry.target
            ));
            lines.push(" 非標準パスのため fcontext ルールの追加が必要です。".to_string());
        }
        crate::selinux::avc::Remedy::Restorecon => {
            lines.push(format!(
                " {} のラベルが剥がれている可能性があります。",
                entry.target
            ));
            lines.push(" restorecon でデフォルトコンテキストに戻すことで解決できます。".to_string());
        }
        crate::selinux::avc::Remedy::Boolean(b) => {
            lines.push(format!(
                " {} Boolean を有効にすることで解決できる可能性があります。",
                b
            ));
        }
        _ => {
            lines.push(format!(
                " ドメイン {} からの {} 操作がポリシーで許可されていません。",
                domain, entry.perm
            ));
            lines.push(" audit2allow でカスタムポリシーを生成する必要があります。".to_string());
        }
    }
    lines
}

/// キー入力からオプションを選択して AuthContext を返す（認証が必要な場合）
pub fn select_option(entry: &AvcEntry, options: &[RemOption], cursor: usize) -> Option<AuthContext> {
    let opt = options.get(cursor)?;
    if !opt.needs_auth || opt.command.is_empty() {
        return None; // 認証不要 or 別フロー
    }
    Some(AuthContext {
        command: opt.command.clone(),
        description: opt.description.clone(),
        prev_screen: Box::new(Screen::AvcDetail(entry.id)),
    })
}
