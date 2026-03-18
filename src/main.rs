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
                    app.update_avc_entries(entries);
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
                    app.append_log(format!("[ERR] AVC ロード失敗: {}", e));
                    app.loading = false;
                    app.status_message = Some(format!("⚠ {}", e));
                }
                AppMsg::SudoResult(res) => match res {
                    SudoResult::Ok => {
                        app.append_log("[OK] コマンド成功".to_string());
                        // 処理対象エントリを処理済みにする（screen_stack に残っている AvcDetail の id を参照）
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
                        app.auth_error = None;
                        app.auth_state.reset();
                        app.screen_stack.truncate(1);
                        app.status_message = Some(app.lang.op_complete().to_string());
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
                        app.cached_password = None;
                        app.auth_state.on_fail();
                        app.auth_error = Some(app.lang.pw_wrong().to_string());
                        *app.password_buf = String::new();
                    }
                    SudoResult::CommandFailed { stderr } => {
                        app.append_log(format!("[ERR] コマンド失敗:\n{}", stderr));
                        app.auth_state.reset();
                        app.pop_screen();
                        app.status_message = Some(
                            app.lang.cmd_failed(stderr.lines().next().unwrap_or(""))
                        );
                    }
                },
                AppMsg::EnforceMode(m) => {
                    app.append_log(format!("[INFO] SELinux モード: {}", m));
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

            // ローディング表示
            if app.loading {
                let spinner =
                    ratatui::widgets::Paragraph::new(app.lang.loading_msg()).style(
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
                                    app.push_screen(Screen::Auth(ctx));
                                    app.prepare_auth();
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
                            description: app.lang.policy_apply_desc().to_string(),
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
        eprintln!("[WARN] 一部の機能に必要なコマンドが見つかりません:");
        for (cmd, pkg) in &missing_optional {
            eprintln!("  {:<14} (パッケージ: {})", cmd, pkg);
        }
        eprintln!("       上記コマンドを使う機能は動作しません。");
        eprintln!();
    }

    if !missing_critical.is_empty() {
        eprintln!("[ERROR] 必須コマンドが見つかりません。seadmin を起動できません:");
        for (cmd, pkg) in &missing_critical {
            eprintln!("  {:<14} (パッケージ: {})", cmd, pkg);
        }
        eprintln!();
        eprintln!("上記パッケージをインストールしてから再実行してください。");
        eprintln!("  例 (Fedora/RHEL): sudo dnf install audit policycoreutils");
        eprintln!("  例 (Debian/Ubuntu): sudo apt install auditd policycoreutils");
        std::process::exit(1);
    }
}
