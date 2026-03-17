mod selinux;
mod sudo;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::selinux::avc::parse_ausearch_output;
use crate::selinux::commands::{ausearch_avc, getenforce, hostname};
use crate::sudo::runner::{run_with_sudo, SudoResult};
use crate::ui::app::{App, AuthContext, Screen};
use crate::ui::screens::avc_detail::{build_options, render as render_detail, select_option};
use crate::ui::screens::avc_list::render as render_avc_list;
use crate::ui::screens::auth_popup::render as render_auth;
use crate::ui::screens::policy_review::render as render_policy_review;
use crate::ui::widgets::{render_footer, render_header, render_log_overlay, render_status};

/// バックグラウンドタスクから UI スレッドへのメッセージ
enum AppMsg {
    AvcLoaded(Vec<crate::selinux::avc::AvcEntry>),
    AvcLoadError(String),
    SudoResult(SudoResult),
    EnforceMode(String),
    Hostname(String),
    PolicyPreview { te: String, pp_path: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    // 起動チェック
    check_env();

    // 端末セットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal).await;

    // 端末を元に戻す
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("エラー: {}", e);
    }
    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    // ログファイルを初期化
    let log_path = {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        std::path::PathBuf::from(home)
            .join(".local/share/seadmin/seadmin.log")
    };
    if let Some(dir) = log_path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(f) => {
            app.log_file = Some(f);
            app.append_log(format!("[INFO] seadmin 起動 (log: {})", log_path.display()));
        }
        Err(e) => {
            app.append_log(format!("[WARN] ログファイルを開けませんでした: {}", e));
        }
    }

    let (tx, mut rx) = mpsc::channel::<AppMsg>(32);

    // 初回ロード
    app.loading = true;
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            match ausearch_avc().await {
                Ok(raw) => {
                    let entries = parse_ausearch_output(&raw);
                    let _ = tx.send(AppMsg::AvcLoaded(entries)).await;
                }
                Err(e) => {
                    let _ = tx.send(AppMsg::AvcLoadError(e.to_string())).await;
                }
            }
        });
    }
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Ok(mode) = getenforce().await {
                let _ = tx.send(AppMsg::EnforceMode(mode)).await;
            }
        });
    }
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Ok(h) = hostname().await {
                let _ = tx.send(AppMsg::Hostname(h)).await;
            }
        });
    }

    // 詳細画面のカーソル状態
    let mut detail_cursor: usize = 0;

    loop {
        // バックグラウンドメッセージを処理
        while let Ok(msg) = rx.try_recv() {
            match msg {
                AppMsg::AvcLoaded(entries) => {
                    app.append_log(format!("[INFO] AVC ロード: {} 件", entries.len()));
                    for e in &entries {
                        let path_note = if e.target.starts_with('/') {
                            format!("path={}", e.target)
                        } else {
                            format!("path={} (絶対パス不明 → restorecon/fcontext 非表示)", e.target)
                        };
                        app.append_log(format!(
                            "[INFO]   #{} proc={} perm={} tclass={} remedy={:?} {}",
                            e.id, e.process, e.perm, e.tclass, e.remedy, path_note
                        ));
                    }
                    app.avc_entries = entries;
                    app.loading = false;
                    if app.selinux_mode == "Disabled" || app.selinux_mode == "UNKNOWN" {
                        app.status_message = Some(
                            "⚠ SELinux が無効です。AVC デナイアルは記録されません。".to_string(),
                        );
                    } else if app.avc_entries.is_empty() {
                        app.status_message = Some("AVC デナイアルはありません".to_string());
                    } else {
                        app.status_message =
                            Some(format!("{} 件の AVC を取得しました", app.avc_entries.len()));
                    }
                }
                AppMsg::AvcLoadError(e) => {
                    app.append_log(format!("[ERR] AVC ロード失敗: {}", e));
                    app.loading = false;
                    app.status_message = Some(format!("⚠ {}", e));
                }
                AppMsg::SudoResult(res) => match res {
                    SudoResult::Ok => {
                        app.append_log("[OK] コマンド成功".to_string());
                        // PolicyReview の .pp ファイルを削除
                        for s in &app.screen_stack {
                            if let Screen::PolicyReview { pp_path, .. } = s {
                                let path = pp_path.clone();
                                tokio::spawn(async move {
                                    let _ = tokio::fs::remove_file(path).await;
                                });
                            }
                        }
                        app.auth_error = None;
                        app.auth_state.reset();
                        app.screen_stack.truncate(1); // 常に AvcList まで戻る
                        app.status_message = Some("操作が完了しました".to_string());
                        // AVC を再取得
                        let tx2 = tx.clone();
                        tokio::spawn(async move {
                            if let Ok(raw) = ausearch_avc().await {
                                let entries = parse_ausearch_output(&raw);
                                let _ = tx2.send(AppMsg::AvcLoaded(entries)).await;
                            }
                        });
                    }
                    SudoResult::AuthFailed => {
                        app.append_log(format!(
                            "[ERR] 認証失敗 ({}/3)",
                            app.auth_state.fail_count + 1
                        ));
                        app.cached_password = None; // キャッシュをクリア
                        app.auth_state.on_fail();
                        app.auth_error = Some("パスワードが正しくありません".to_string());
                        *app.password_buf = String::new();
                    }
                    SudoResult::CommandFailed { stderr } => {
                        app.append_log(format!("[ERR] コマンド失敗:\n{}", stderr));
                        app.auth_state.reset();
                        app.pop_screen();
                        app.status_message = Some(format!(
                            "コマンド失敗: {}",
                            stderr.lines().next().unwrap_or("")
                        ));
                    }
                },
                AppMsg::EnforceMode(m) => {
                    app.append_log(format!("[INFO] SELinux モード: {}", m));
                    // すでに AVC ロード完了済みで Disabled が判明した場合は警告を更新
                    if m == "Disabled" && !app.loading {
                        app.status_message = Some(
                            "⚠ SELinux が無効です。AVC デナイアルは記録されません。".to_string(),
                        );
                    }
                    app.selinux_mode = m;
                }
                AppMsg::Hostname(h) => {
                    app.hostname = h;
                }
                AppMsg::PolicyPreview { te, pp_path } => {
                    let lines = te.lines().count();
                    app.append_log(format!(
                        "[INFO] audit2allow 生成完了: {} 行, pp={}",
                        lines, pp_path
                    ));
                    app.loading = false;
                    app.policy_review_scroll = 0;
                    app.push_screen(Screen::PolicyReview { te, pp_path });
                }
            }
        }

        // 描画
        terminal.draw(|f| {
            let size = f.area();

            // ヘッダー(3行) / コンテンツ / フッター(1行) / ステータス(1行)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(size);

            render_header(f, chunks[0], &app.selinux_mode, &app.hostname);

            // コンテンツエリア
            match app.current_screen().clone() {
                Screen::AvcList => {
                    let hint = "↑↓/jk:移動  Enter:詳細  /:フィルタ  r:更新  l:ログ  q:終了";
                    render_avc_list(f, chunks[1], &mut app);
                    render_footer(f, chunks[2], hint);
                }
                Screen::AvcDetail(id) => {
                    if let Some(entry) = app.avc_entries.iter().find(|e| e.id == id).cloned() {
                        let opts = build_options(&entry);
                        render_detail(f, chunks[1], &entry, detail_cursor, &opts);
                        let hint = "A-F:対処選択  Esc/←:戻る  Enter:確認へ";
                        render_footer(f, chunks[2], hint);
                    }
                }
                Screen::PolicyReview { te, .. } => {
                    let hint = "↑↓/jk:スクロール  Enter:適用  Esc:キャンセル";
                    render_policy_review(f, chunks[1], &te, app.policy_review_scroll);
                    render_footer(f, chunks[2], hint);
                }
                Screen::Auth(ref ctx) => {
                    // 背景として前画面を描画
                    match *ctx.prev_screen.clone() {
                        Screen::AvcDetail(id) => {
                            if let Some(entry) =
                                app.avc_entries.iter().find(|e| e.id == id).cloned()
                            {
                                let opts = build_options(&entry);
                                render_detail(f, chunks[1], &entry, detail_cursor, &opts);
                            }
                        }
                        Screen::PolicyReview { ref te, .. } => {
                            render_policy_review(
                                f,
                                chunks[1],
                                te,
                                app.policy_review_scroll,
                            );
                        }
                        _ => {
                            render_avc_list(f, chunks[1], &mut app);
                        }
                    }
                    let hint = "Enter:実行  Esc:キャンセル";
                    render_footer(f, chunks[2], hint);
                    render_auth(f, size, &app, ctx);
                }
            }

            // ステータスバー
            if let Some(msg) = &app.status_message.clone() {
                let is_err = msg.starts_with('⚠');
                render_status(f, chunks[3], msg, is_err);
            }

            // ローディング表示
            if app.loading {
                let spinner =
                    ratatui::widgets::Paragraph::new(" ⏳ AVC ログを読み込み中...").style(
                        ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                    );
                f.render_widget(spinner, chunks[3]);
            }

            // ログオーバーレイ
            if app.show_log {
                render_log_overlay(f, size, &app.log, app.log_scroll);
            }
        })?;

        // イベント処理（50ms タイムアウト）
        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            // Ctrl+C は常に終了
            if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                break;
            }

            // l キーでログオーバーレイをトグル（全画面共通）
            if key.code == KeyCode::Char('l') && !app.avc_filter_active {
                app.show_log = !app.show_log;
                app.log_scroll = 0;
                continue;
            }

            // ログオーバーレイ表示中はスクロールのみ受け付ける
            if app.show_log {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.log_scroll = app.log_scroll.saturating_add(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.log_scroll = app.log_scroll.saturating_sub(1);
                    }
                    _ => {}
                }
                continue;
            }

            match app.current_screen().clone() {
                Screen::AvcList => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.status_message = None;
                        app.cursor_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.status_message = None;
                        app.cursor_down();
                    }
                    KeyCode::Enter => {
                        if let Some(entry) = app.selected_avc() {
                            let id = entry.id;
                            detail_cursor = 0;
                            app.push_screen(Screen::AvcDetail(id));
                        }
                    }
                    KeyCode::Char('r') => {
                        app.loading = true;
                        app.status_message = None;
                        let tx2 = tx.clone();
                        tokio::spawn(async move {
                            match ausearch_avc().await {
                                Ok(raw) => {
                                    let entries = parse_ausearch_output(&raw);
                                    let _ = tx2.send(AppMsg::AvcLoaded(entries)).await;
                                }
                                Err(e) => {
                                    let _ = tx2.send(AppMsg::AvcLoadError(e.to_string())).await;
                                }
                            }
                        });
                    }
                    KeyCode::Char('/') => {
                        app.avc_filter_active = true;
                        app.avc_filter.clear();
                    }
                    KeyCode::Esc if app.avc_filter_active => {
                        app.avc_filter_active = false;
                        app.avc_filter.clear();
                    }
                    KeyCode::Backspace if app.avc_filter_active => {
                        app.avc_filter.pop();
                    }
                    KeyCode::Char(c) if app.avc_filter_active => {
                        app.avc_filter.push(c);
                        app.avc_cursor = 0;
                    }
                    _ => {}
                },

                Screen::AvcDetail(id) => {
                    let entry_clone = app.avc_entries.iter().find(|e| e.id == id).cloned();
                    if let Some(entry) = entry_clone {
                        let opts = build_options(&entry);
                        let opts_len = opts.len();
                        match key.code {
                            KeyCode::Esc | KeyCode::Left => {
                                app.pop_screen();
                                detail_cursor = 0;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if detail_cursor > 0 {
                                    detail_cursor -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if detail_cursor < opts_len.saturating_sub(1) {
                                    detail_cursor += 1;
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(ctx) = select_option(&entry, &opts, detail_cursor) {
                                    app.push_screen(Screen::Auth(ctx));
                                    app.prepare_auth();
                                } else {
                                    let opt = &opts[detail_cursor];
                                    if opt.key == 'F' {
                                        if let Some(e) =
                                            app.avc_entries.iter_mut().find(|e| e.id == id)
                                        {
                                            e.resolved = true;
                                        }
                                        app.pop_screen();
                                        app.status_message =
                                            Some("無視リストに追加しました".to_string());
                                    } else if opt.command.is_empty() {
                                        // audit2allow フロー
                                        let raw = entry.raw_lines.clone();
                                        let module = format!("local_{}", entry.id);
                                        app.append_log(format!("[CMD] audit2allow -M {} ({} 行のログを入力)", module, raw.len()));
                                        let tx2 = tx.clone();
                                        app.loading = true;
                                        tokio::spawn(async move {
                                            match crate::selinux::commands::audit2allow_generate(
                                                &raw, &module,
                                            )
                                            .await
                                            {
                                                Ok((te, pp_path)) => {
                                                    let _ = tx2
                                                        .send(AppMsg::PolicyPreview { te, pp_path })
                                                        .await;
                                                }
                                                Err(e) => {
                                                    let _ = tx2
                                                        .send(AppMsg::AvcLoadError(e.to_string()))
                                                        .await;
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                            KeyCode::Char(c) => {
                                // キー直接選択 (A, B, C, ...)
                                let c_up = c.to_ascii_uppercase();
                                if let Some(pos) = opts.iter().position(|o| o.key == c_up) {
                                    detail_cursor = pos;
                                    let opt = &opts[pos];
                                    if let Some(ctx) = select_option(&entry, &opts, pos) {
                                        app.push_screen(Screen::Auth(ctx));
                                        *app.password_buf = String::new();
                                        app.auth_error = None;
                                    } else if opt.key == 'F' {
                                        if let Some(e) =
                                            app.avc_entries.iter_mut().find(|e| e.id == id)
                                        {
                                            e.resolved = true;
                                        }
                                        app.pop_screen();
                                        app.status_message =
                                            Some("無視リストに追加しました".to_string());
                                    } else if opt.command.is_empty() {
                                        let raw = entry.raw_lines.clone();
                                        let module = format!("local_{}", entry.id);
                                        let tx2 = tx.clone();
                                        app.loading = true;
                                        tokio::spawn(async move {
                                            match crate::selinux::commands::audit2allow_generate(
                                                &raw, &module,
                                            )
                                            .await
                                            {
                                                Ok((te, pp_path)) => {
                                                    let _ = tx2
                                                        .send(AppMsg::PolicyPreview { te, pp_path })
                                                        .await;
                                                }
                                                Err(e) => {
                                                    let _ = tx2
                                                        .send(AppMsg::AvcLoadError(e.to_string()))
                                                        .await;
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Screen::PolicyReview { te, pp_path } => match key.code {
                    KeyCode::Esc => {
                        // キャンセル時は .pp を削除
                        tokio::spawn(async move {
                            let _ = tokio::fs::remove_file(pp_path).await;
                        });
                        app.pop_screen();
                    }
                    KeyCode::Enter => {
                        // semodule -i で適用 → Auth ポップアップへ
                        let cmd = vec![
                            "semodule".to_string(),
                            "-i".to_string(),
                            pp_path.clone(),
                        ];
                        app.push_screen(Screen::Auth(AuthContext {
                            command: cmd,
                            description: "生成したポリシーモジュールをシステムに適用します。"
                                .to_string(),
                            prev_screen: Box::new(Screen::PolicyReview { te, pp_path }),
                        }));
                        *app.password_buf = String::new();
                        app.auth_error = None;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.policy_review_scroll =
                            app.policy_review_scroll.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.policy_review_scroll += 1;
                    }
                    _ => {}
                },

                Screen::Auth(ctx) => {
                    if app.auth_state.is_locked() {
                        if key.code == KeyCode::Esc {
                            app.pop_screen();
                            *app.password_buf = String::new();
                            app.auth_error = None;
                        }
                        continue;
                    }
                    match key.code {
                        KeyCode::Esc => {
                            app.pop_screen();
                            *app.password_buf = String::new();
                            app.auth_error = None;
                        }
                        KeyCode::Enter => {
                            let pw = app.password_buf.clone();
                            // 認証成功を期待してキャッシュに保存（失敗時にクリア）
                            app.cached_password = Some(pw.clone());
                            *app.password_buf = String::new();
                            let cmd = ctx.command.clone();
                            app.append_log(format!("[CMD] sudo {}", cmd.join(" ")));
                            let tx2 = tx.clone();
                            tokio::spawn(async move {
                                let cmd_refs: Vec<&str> =
                                    cmd.iter().map(String::as_str).collect();
                                let res = run_with_sudo(&cmd_refs, pw).await;
                                match res {
                                    Ok(r) => {
                                        let _ = tx2.send(AppMsg::SudoResult(r)).await;
                                    }
                                    Err(e) => {
                                        let _ = tx2
                                            .send(AppMsg::SudoResult(SudoResult::CommandFailed {
                                                stderr: e.to_string(),
                                            }))
                                            .await;
                                    }
                                }
                            });
                        }
                        KeyCode::Backspace => {
                            app.password_buf.pop();
                        }
                        KeyCode::Char(c) => {
                            app.password_buf.push(c);
                        }
                        _ => {}
                    }
                }

            }
        }
    }

    Ok(())
}

/// 起動時の環境チェック
fn check_env() {
    let lang = std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .unwrap_or_default();
    if !lang.to_uppercase().contains("UTF") {
        eprintln!(
            "警告: ロケールが UTF-8 ではない可能性があります（LANG={}）。",
            lang
        );
        eprintln!("日本語が正しく表示されない場合は LANG=ja_JP.UTF-8 を設定してください。");
    }
}
