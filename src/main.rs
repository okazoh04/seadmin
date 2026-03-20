/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

mod i18n;
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

use crate::selinux::avc::{parse_ausearch_output, Remedy};
use crate::selinux::commands::{audit2why, ausearch_avc, getenforce, hostname, sepolicy_bool_desc, semodule_list_full, PolicyModule};
use crate::sudo::runner::{run_with_sudo, SudoResult};
use crate::ui::app::{App, AuthContext, Screen};
use crate::ui::screens::avc_detail::{build_options, render as render_detail, select_option};
use crate::ui::screens::avc_list::render as render_avc_list;
use crate::ui::screens::auth_popup::render as render_auth;
use crate::ui::screens::policy_review::render as render_policy_review;
use crate::ui::screens::module_list::render as render_module_list;
use crate::ui::widgets::{render_footer, render_header, render_log_overlay, render_status};

/// バックグラウンドタスクから UI スレッドへのメッセージ
enum AppMsg {
    AvcLoaded(Vec<crate::selinux::avc::AvcEntry>),
    AvcLoadError(String),
    SudoResult(SudoResult),
    EnforceMode(String),
    Hostname(String),
    PolicyPreview { te: String, pp_path: String },
    /// audit2why による Remedy 更新: (entry_id, new_remedy)
    Audit2WhyDone(usize, crate::selinux::avc::Remedy),
    /// sepolicy booleans による説明文: (entry_id, description)
    BoolDescDone(usize, String),
    /// semodule -lfull の結果
    ModuleListLoaded(Vec<PolicyModule>),
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
        eprintln!("Error: {}", e);
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
            app.append_log(app.lang.log_startup(&log_path.display().to_string()));
        }
        Err(e) => {
            app.append_log(app.lang.log_file_open_error(&e.to_string()));
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
        app.tick = app.tick.wrapping_add(1);

        // バックグラウンドメッセージを処理
        while let Ok(msg) = rx.try_recv() {
            match msg {
                AppMsg::AvcLoaded(entries) => {
                    app.append_log(app.lang.log_avc_loaded_n(entries.len()));
                    for e in &entries {
                        let path_note = if e.target.starts_with('/') {
                            format!("path={}", e.target)
                        } else {
                            app.lang.log_path_no_abs(&e.target)
                        };
                        app.append_log(format!(
                            "[INFO]   #{} proc={} perm={} tclass={} remedy={:?} {}",
                            e.id, e.process, e.perm, e.tclass, e.remedy, path_note
                        ));
                    }
                    app.update_avc_entries(entries);
                    // audit2why で Remedy を精緻化（バックグラウンド）
                    for entry in &app.avc_entries {
                        let raw = entry.raw_lines.clone();
                        let id  = entry.id;
                        let tx2 = tx.clone();
                        tokio::spawn(async move {
                            if let Some((category, bool_name)) = audit2why(&raw).await {
                                let remedy = match category.as_str() {
                                    "BOOLEAN" if !bool_name.is_empty() => Remedy::Boolean(bool_name),
                                    "BADTCON"   => Remedy::Restorecon,
                                    "TERULE"    => Remedy::CustomPolicy,
                                    _           => return,
                                };
                                let _ = tx2.send(AppMsg::Audit2WhyDone(id, remedy)).await;
                            }
                        });
                    }
                    app.loading = false;
                    if app.selinux_mode == "Disabled" || app.selinux_mode == "UNKNOWN" {
                        app.status_message = Some(app.lang.selinux_disabled().to_string());
                    } else if app.avc_entries.is_empty() {
                        app.status_message = Some(app.lang.no_avc().to_string());
                    } else {
                        app.status_message =
                            Some(app.lang.avc_loaded(app.avc_entries.len()));
                    }
                }
                AppMsg::AvcLoadError(e) => {
                    app.append_log(app.lang.log_avc_load_error(&e));
                    app.loading = false;
                    app.status_message = Some(format!("⚠ {}", e));
                }
                AppMsg::ModuleListLoaded(modules) => {
                    app.module_list = modules;
                    app.module_cursor = 0;
                    app.loading = false;
                }
                AppMsg::SudoResult(res) => match res {
                    SudoResult::Ok => {
                        app.loading = false;
                        app.loading_label = None;
                        app.append_log(app.lang.log_cmd_ok().to_string());
                        let pending_ctx = app.pending_auth_ctx.take();
                        app.auth_error = None;
                        app.auth_state.reset();

                        // ModuleList 画面からの操作（モジュール削除）かチェック
                        let is_module_op = app.screen_stack.iter().any(|s| matches!(s, Screen::ModuleList));
                        if is_module_op {
                            // 削除したモジュール名を取得してメッセージ表示
                            let deleted_name = pending_ctx.as_ref()
                                .and_then(|ctx| ctx.command.get(4))
                                .cloned()
                                .unwrap_or_default();
                            app.status_message = Some(if deleted_name.is_empty() {
                                app.lang.op_complete().to_string()
                            } else {
                                app.lang.module_deleted(&deleted_name)
                            });
                            // AvcList に戻り、モジュール一覧を再取得
                            app.screen_stack.truncate(1);
                            let tx2 = tx.clone();
                            tokio::spawn(async move {
                                if let Ok(modules) = semodule_list_full().await {
                                    let _ = tx2.send(AppMsg::ModuleListLoaded(modules)).await;
                                }
                            });
                        } else {
                            app.status_message = Some(app.lang.op_complete().to_string());
                            // 処理対象エントリを処理済みにする
                            let resolved_id = app.screen_stack.iter().find_map(|s| {
                                if let Screen::AvcDetail(id) = s { Some(*id) } else { None }
                            });
                            if let Some(id) = resolved_id {
                                app.mark_resolved(id);
                            }
                            // PolicyReview の .pp ファイルを削除
                            for s in &app.screen_stack {
                                if let Screen::PolicyReview { pp_path, .. } = s {
                                    let path = pp_path.clone();
                                    tokio::spawn(async move {
                                        let _ = tokio::fs::remove_file(path).await;
                                    });
                                }
                            }
                            app.screen_stack.truncate(1);
                            // AVC を再取得
                            let tx2 = tx.clone();
                            tokio::spawn(async move {
                                if let Ok(raw) = ausearch_avc().await {
                                    let entries = parse_ausearch_output(&raw);
                                    let _ = tx2.send(AppMsg::AvcLoaded(entries)).await;
                                }
                            });
                        }
                    }
                    SudoResult::AuthFailed => {
                        app.loading = false;
                        app.loading_label = None;
                        app.append_log(app.lang.log_auth_failed((app.auth_state.fail_count + 1) as u32));
                        app.cached_password = None;
                        app.auth_state.on_fail();
                        app.auth_error = Some(app.lang.pw_wrong().to_string());
                        *app.password_buf = String::new();
                        // キャッシュ認証でスキップしていた場合は Auth 画面を表示
                        if let Some(ctx) = app.pending_auth_ctx.take() {
                            app.push_screen(Screen::Auth(ctx));
                        }
                    }
                    SudoResult::CommandFailed { stderr } => {
                        app.loading = false;
                        app.loading_label = None;
                        app.append_log(app.lang.log_cmd_failed_msg(&stderr));
                        app.pending_auth_ctx = None;
                        app.auth_state.reset();
                        app.pop_screen();
                        app.status_message = Some(
                            app.lang.cmd_failed(stderr.lines().next().unwrap_or(""))
                        );
                    }
                },
                AppMsg::Audit2WhyDone(id, remedy) => {
                    app.append_log(format!("[INFO] audit2why #{}: {:?}", id, remedy));
                    // Boolean の場合は sepolicy で説明文を取得
                    if let Remedy::Boolean(ref bool_name) = remedy {
                        let bool_name = bool_name.clone();
                        let tx2 = tx.clone();
                        tokio::spawn(async move {
                            if let Some(desc) = sepolicy_bool_desc(&bool_name).await {
                                let _ = tx2.send(AppMsg::BoolDescDone(id, desc)).await;
                            }
                        });
                    }
                    if let Some(entry) = app.avc_entries.iter_mut().find(|e| e.id == id) {
                        entry.remedy = remedy;
                    }
                }
                AppMsg::BoolDescDone(id, desc) => {
                    if let Some(entry) = app.avc_entries.iter_mut().find(|e| e.id == id) {
                        entry.bool_description = Some(desc);
                    }
                }
                AppMsg::EnforceMode(m) => {
                    app.append_log(app.lang.log_selinux_mode(&m));
                    if m == "Disabled" && !app.loading {
                        app.status_message = Some(app.lang.selinux_disabled().to_string());
                    }
                    app.selinux_mode = m;
                }
                AppMsg::Hostname(h) => {
                    app.hostname = h;
                }
                AppMsg::PolicyPreview { te, pp_path } => {
                    let lines = te.lines().count();
                    app.append_log(app.lang.log_audit2allow_done(lines, &pp_path));
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

            // 毎フレーム全画面クリア（画面遷移・Auth popup 背景などの残像を確実に除去）
            f.render_widget(ratatui::widgets::Clear, size);

            render_header(f, chunks[0], &app.selinux_mode, &app.hostname, &app.lang);

            // コンテンツエリア
            match app.current_screen().clone() {
                Screen::AvcList => {
                    let hint = app.lang.hint_avc_list();
                    render_avc_list(f, chunks[1], &mut app);
                    render_footer(f, chunks[2], hint);
                }
                Screen::AvcDetail(id) => {
                    if let Some(entry) = app.avc_entries.iter().find(|e| e.id == id).cloned() {
                        let opts = build_options(&entry, &app.lang);
                        render_detail(f, chunks[1], &entry, detail_cursor, &opts, &app.lang);
                        let hint = app.lang.hint_avc_detail();
                        render_footer(f, chunks[2], hint);
                    }
                }
                Screen::PolicyReview { te, .. } => {
                    let hint = app.lang.hint_policy_review();
                    render_policy_review(f, chunks[1], &te, app.policy_review_scroll, &app.lang);
                    render_footer(f, chunks[2], hint);
                }
                Screen::ModuleList => {
                    let hint = app.lang.hint_module_list();
                    render_module_list(f, chunks[1], &app);
                    render_footer(f, chunks[2], hint);
                }
                Screen::Auth(ref ctx) => {
                    // 背景として前画面を描画
                    match *ctx.prev_screen.clone() {
                        Screen::AvcDetail(id) => {
                            if let Some(entry) =
                                app.avc_entries.iter().find(|e| e.id == id).cloned()
                            {
                                let opts = build_options(&entry, &app.lang);
                                render_detail(f, chunks[1], &entry, detail_cursor, &opts, &app.lang);
                            }
                        }
                        Screen::PolicyReview { ref te, .. } => {
                            render_policy_review(
                                f,
                                chunks[1],
                                te,
                                app.policy_review_scroll,
                                &app.lang,
                            );
                        }
                        Screen::ModuleList => {
                            render_module_list(f, chunks[1], &app);
                        }
                        _ => {
                            render_avc_list(f, chunks[1], &mut app);
                        }
                    }
                    let hint = app.lang.hint_auth();
                    render_footer(f, chunks[2], hint);
                    render_auth(f, size, &app, ctx);
                }
            }

            // ステータスバー
            if let Some(msg) = &app.status_message.clone() {
                let is_err = msg.starts_with('⚠');
                render_status(f, chunks[3], msg, is_err);
            }

            // ローディング表示（スピナーアニメーション）
            if app.loading {
                let label = app.loading_label.as_deref();
                let spinner_text = app.lang.loading_spinner(app.tick / 3, label);
                let spinner = ratatui::widgets::Paragraph::new(spinner_text).style(
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                );
                f.render_widget(spinner, chunks[3]);
            }

            // ログオーバーレイ
            if app.show_log {
                render_log_overlay(f, size, &app.log, app.log_scroll, &app.lang);
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
                    KeyCode::Char('m') => {
                        app.module_cursor = 0;
                        app.push_screen(Screen::ModuleList);
                        app.loading = true;
                        let tx2 = tx.clone();
                        tokio::spawn(async move {
                            match semodule_list_full().await {
                                Ok(modules) => {
                                    let _ = tx2.send(AppMsg::ModuleListLoaded(modules)).await;
                                }
                                Err(_) => {
                                    let _ = tx2.send(AppMsg::ModuleListLoaded(Vec::new())).await;
                                }
                            }
                        });
                    }
                    _ => {}
                },

                Screen::AvcDetail(id) => {
                    let entry_clone = app.avc_entries.iter().find(|e| e.id == id).cloned();
                    if let Some(entry) = entry_clone {
                        let opts = build_options(&entry, &app.lang);
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
                                    if !try_sudo_with_cache(&mut app, ctx.clone(), &tx) {
                                        app.push_screen(Screen::Auth(ctx));
                                        app.prepare_auth();
                                    }
                                } else {
                                    let opt = &opts[detail_cursor];
                                    if opt.key == 'F' {
                                        app.mark_resolved(id);
                                        app.pop_screen();
                                        app.status_message =
                                            Some(app.lang.ignored().to_string());
                                    } else if opt.command.is_empty() {
                                        // audit2allow フロー
                                        let raw = entry.raw_lines.clone();
                                        let module = make_module_name(&entry);
                                        app.append_log(app.lang.log_audit2allow_cmd(&module, raw.len()));
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
                                        if !try_sudo_with_cache(&mut app, ctx.clone(), &tx) {
                                            app.push_screen(Screen::Auth(ctx));
                                            app.prepare_auth();
                                        }
                                    } else if opt.key == 'F' {
                                        app.mark_resolved(id);
                                        app.pop_screen();
                                        app.status_message =
                                            Some(app.lang.ignored().to_string());
                                    } else if opt.command.is_empty() {
                                        let raw = entry.raw_lines.clone();
                                        let module = make_module_name(&entry);
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

                Screen::ModuleList => match key.code {
                    KeyCode::Esc => {
                        app.pop_screen();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.module_cursor > 0 {
                            app.module_cursor -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.module_cursor < app.module_list.len().saturating_sub(1) {
                            app.module_cursor += 1;
                        }
                    }
                    KeyCode::Char('d') => {
                        if let Some(m) = app.module_list.get(app.module_cursor).cloned() {
                            let cmd = vec![
                                "semodule".to_string(),
                                "-X".to_string(),
                                m.priority.to_string(),
                                "-r".to_string(),
                                m.name.clone(),
                            ];
                            let desc = app.lang.module_delete_desc(&m.name);
                            let ctx = AuthContext {
                                command: cmd,
                                description: desc,
                                prev_screen: Box::new(Screen::ModuleList),
                            };
                            if !try_sudo_with_cache(&mut app, ctx.clone(), &tx) {
                                app.push_screen(Screen::Auth(ctx));
                                app.prepare_auth();
                            }
                        }
                    }
                    _ => {}
                },

                Screen::PolicyReview { te, pp_path } => match key.code {
                    KeyCode::Esc => {
                        // キャンセル時は .pp を削除
                        tokio::spawn(async move {
                            let _ = tokio::fs::remove_file(pp_path).await;
                        });
                        app.pop_screen();
                    }
                    KeyCode::Enter => {
                        // semodule -i で適用 → キャッシュがあれば直接実行、なければ Auth ポップアップへ
                        let cmd = vec![
                            "semodule".to_string(),
                            "-X".to_string(),
                            "300".to_string(),
                            "-i".to_string(),
                            pp_path.clone(),
                        ];
                        let ctx = AuthContext {
                            command: cmd,
                            description: app.lang.policy_apply_desc().to_string(),
                            prev_screen: Box::new(Screen::PolicyReview { te, pp_path }),
                        };
                        if !try_sudo_with_cache(&mut app, ctx.clone(), &tx) {
                            app.push_screen(Screen::Auth(ctx));
                            app.prepare_auth();
                        }
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
                            app.cached_password = Some(pw.clone());
                            *app.password_buf = String::new();
                            let cmd = ctx.command.clone();
                            app.loading = true;
                            app.loading_label = Some(ctx.description.clone());
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
/// AVC エントリから安定したポリシーモジュール名を生成する
///
/// `entry.id` はリロードのたびに変わるため、エントリの内容（stype/ttype/tclass/perm）
/// から決定論的な名前を生成する。同じ AVC には常に同じモジュール名が割り当てられるため、
/// 重複適用しても上書きにならず、別エントリのモジュールを誤って消すことがない。
///
/// SELinux モジュール名は `[a-zA-Z][a-zA-Z0-9_]*` の形式が必要。
fn make_module_name(entry: &crate::selinux::avc::AvcEntry) -> String {
    let stype = entry.scontext.split(':').nth(2).unwrap_or("unknown");
    let ttype = entry.tcontext.split(':').nth(2).unwrap_or("unknown");
    // perm はスペース区切りの場合があるのでアンダースコアに変換
    let perm = entry.perm.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    // SELinux モジュール名の長さ制限に備えて 64 文字に切り詰める
    let raw = format!("local_{}_{}_{}_{}",
        stype, ttype, entry.tclass, perm);
    if raw.len() <= 64 {
        raw
    } else {
        raw[..64].to_string()
    }
}

/// キャッシュ済みパスワードがあれば Auth 画面をスキップして直接 sudo 実行する。
/// キャッシュがあれば true（コマンド発行済み）、なければ false（呼び出し元が Auth 画面を表示すべき）を返す。
fn try_sudo_with_cache(app: &mut App, ctx: AuthContext, tx: &mpsc::Sender<AppMsg>) -> bool {
    let Some(cached_pw) = app.cached_password.clone() else {
        return false;
    };
    let cmd = ctx.command.clone();
    let desc = ctx.description.clone();
    app.append_log(app.lang.log_sudo_cached(&cmd.join(" ")));
    app.loading = true;
    app.loading_label = Some(desc);
    app.pending_auth_ctx = Some(ctx);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        let cmd_refs: Vec<&str> = cmd.iter().map(String::as_str).collect();
        let res = run_with_sudo(&cmd_refs, cached_pw).await;
        let msg = match res {
            Ok(r) => AppMsg::SudoResult(r),
            Err(e) => AppMsg::SudoResult(SudoResult::CommandFailed { stderr: e.to_string() }),
        };
        let _ = tx2.send(msg).await;
    });
    true
}

fn check_env() {
    let lang_val = std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .unwrap_or_default();
    if !lang_val.to_uppercase().contains("UTF") {
        let lang = crate::i18n::detect_lang();
        eprintln!("{}", lang.warn_locale_not_utf8(&lang_val));
    }

    check_deps();
}

/// コマンドが PATH 上に存在するか確認する
fn is_in_path(cmd: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path| {
            std::env::split_paths(&path).any(|dir| dir.join(cmd).is_file())
        })
        .unwrap_or(false)
}

/// 依存コマンドの存在確認。不足があれば警告・エラーを出力し、
/// 必須コマンドが欠けている場合はプロセスを終了する。
fn check_deps() {
    let lang = crate::i18n::detect_lang();

    // (コマンド名, パッケージヒント, 必須かどうか)
    let deps: &[(&str, &str, bool)] = &[
        ("ausearch",    "audit / auditd",                     true),
        ("getenforce",  "policycoreutils",                    true),
        ("sudo",        "sudo",                               true),
        ("audit2allow", "policycoreutils-python-utils / policycoreutils-dev", false),
        ("semodule",    "policycoreutils",                    false),
        ("restorecon",  "policycoreutils",                    false),
        ("semanage",    "policycoreutils-python-utils / policycoreutils-dev", false),
        ("setsebool",   "policycoreutils",                    false),
    ];

    let mut missing_critical: Vec<(&str, &str)> = Vec::new();
    let mut missing_optional: Vec<(&str, &str)> = Vec::new();

    for &(cmd, pkg, critical) in deps {
        if !is_in_path(cmd) {
            if critical {
                missing_critical.push((cmd, pkg));
            } else {
                missing_optional.push((cmd, pkg));
            }
        }
    }

    if !missing_optional.is_empty() {
        eprintln!("{}", lang.warn_missing_opt_hdr());
        for (cmd, pkg) in &missing_optional {
            eprintln!("{}", lang.warn_missing_cmd(cmd, pkg));
        }
        eprintln!("{}", lang.warn_missing_opt_ftr());
        eprintln!();
    }

    if !missing_critical.is_empty() {
        eprintln!("{}", lang.err_missing_crit_hdr());
        for (cmd, pkg) in &missing_critical {
            eprintln!("{}", lang.warn_missing_cmd(cmd, pkg));
        }
        eprintln!();
        eprintln!("{}", lang.err_install_hint());
        std::process::exit(1);
    }
}
