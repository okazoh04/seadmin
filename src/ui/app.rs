/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use crate::i18n::Lang;
use crate::selinux::avc::AvcEntry;
use crate::selinux::commands::PolicyModule;
use std::collections::VecDeque;
use std::io::Write;
use zeroize::Zeroizing;

/// アプリケーション全体の画面状態
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    AvcList,
    AvcDetail(usize), // AvcEntry の id
    Auth(AuthContext),
    /// audit2allow で生成したポリシーのレビュー・適用確認画面
    PolicyReview {
        te: String,
        pp_path: String,
    },
    /// semodule -lfull で取得したモジュール管理画面
    ModuleList,
    /// パス手動入力ポップアップ（FileContext / Restorecon でパスが不明な場合）
    PathInput(usize), // AvcEntry の id
}

/// 認証ポップアップを表示する際のコンテキスト
#[derive(Debug, Clone, PartialEq)]
pub struct AuthContext {
    /// 認証後に実行するコマンド（表示・実行両用）
    pub command: Vec<String>,
    /// ユーザー向け説明文
    pub description: String,
    /// 前の画面（認証キャンセル・完了時に戻る先）
    pub prev_screen: Box<Screen>,
}

/// sudo 認証のロックアウト状態
#[derive(Debug, Default)]
pub struct AuthState {
    pub fail_count: u8,
    pub locked_until: Option<std::time::Instant>,
}

impl AuthState {
    pub fn is_locked(&self) -> bool {
        match self.locked_until {
            Some(t) => std::time::Instant::now() < t,
            None => false,
        }
    }

    pub fn lock_remaining_secs(&self) -> u64 {
        match self.locked_until {
            Some(t) => {
                let now = std::time::Instant::now();
                if now < t {
                    (t - now).as_secs()
                } else {
                    0
                }
            }
            None => 0,
        }
    }

    pub fn on_fail(&mut self) {
        self.fail_count += 1;
        if self.fail_count >= 3 {
            self.locked_until =
                Some(std::time::Instant::now() + std::time::Duration::from_secs(60));
        }
    }

    pub fn reset(&mut self) {
        self.fail_count = 0;
        self.locked_until = None;
    }
}

/// アプリケーション全体の状態
pub struct App {
    pub screen_stack: Vec<Screen>,
    pub avc_entries: Vec<AvcEntry>,
    /// コマンド実行済みエントリのキー集合（ausearch 再取得後も resolved 状態を維持するため）
    /// キー形式: "scontext|tcontext|tclass|perm"
    pub resolved_keys: std::collections::HashSet<String>,
    pub avc_cursor: usize,
    pub avc_filter: String,
    pub avc_filter_active: bool,
    pub selinux_mode: String,
    pub hostname: String,
    pub auth_state: AuthState,
    /// パスワード入力バッファ（zeroize 対応）
    pub password_buf: Zeroizing<String>,
    /// セッション内パスワードキャッシュ（認証成功後に保持、失敗時にクリア）
    pub cached_password: Option<Zeroizing<String>>,
    /// キャッシュ認証で実行中のコンテキスト（Auth 画面をスキップした場合に保持）
    pub pending_auth_ctx: Option<AuthContext>,
    pub auth_error: Option<String>,
    pub status_message: Option<String>,
    pub loading: bool,
    /// ローディング中の説明ラベル（None のとき lang.loading_msg() を使用）
    pub loading_label: Option<String>,
    /// ループカウンタ（スピナーアニメーション用）
    pub tick: u64,
    /// PolicyReview 画面のスクロール位置
    pub policy_review_scroll: usize,
    /// 操作ログ（最大 500 件、新しいものが末尾）
    pub log: VecDeque<String>,
    pub log_scroll: usize,
    pub show_log: bool,
    /// ログファイル（~/.local/share/seadmin/seadmin.log）
    pub log_file: Option<std::fs::File>,
    /// 表示言語（起動時に LANG 環境変数から決定）
    pub lang: Lang,
    /// ポリシーモジュール一覧
    pub module_list: Vec<PolicyModule>,
    pub module_cursor: usize,
    /// PathInput ポップアップのテキスト入力バッファ
    pub path_input_buf: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen_stack: vec![Screen::AvcList],
            avc_entries: Vec::new(),
            resolved_keys: std::collections::HashSet::new(),
            avc_cursor: 0,
            avc_filter: String::new(),
            avc_filter_active: false,
            selinux_mode: "UNKNOWN".to_string(),
            hostname: String::new(),
            auth_state: AuthState::default(),
            password_buf: Zeroizing::new(String::new()),
            cached_password: None,
            pending_auth_ctx: None,
            auth_error: None,
            status_message: None,
            loading: false,
            loading_label: None,
            tick: 0,
            policy_review_scroll: 0,
            log: VecDeque::new(),
            log_scroll: 0,
            show_log: false,
            log_file: None,
            lang: crate::i18n::detect_lang(),
            module_list: Vec::new(),
            module_cursor: 0,
            path_input_buf: String::new(),
        }
    }

    pub fn append_log(&mut self, msg: impl Into<String>) {
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let line = format!("[{}] {}", ts, msg.into());
        self.log.push_back(line.clone());
        if self.log.len() > 500 {
            self.log.pop_front();
        }
        if let Some(ref mut f) = self.log_file {
            let _ = writeln!(f, "{}", line);
        }
    }

    /// Auth 画面を開く際にキャッシュがあれば password_buf に自動入力する
    pub fn prepare_auth(&mut self) {
        *self.password_buf = match &self.cached_password {
            Some(cached) => (**cached).clone(),
            None => String::new(),
        };
        self.auth_error = None;
    }

    pub fn current_screen(&self) -> &Screen {
        self.screen_stack.last().unwrap_or(&Screen::AvcList)
    }

    pub fn push_screen(&mut self, screen: Screen) {
        self.screen_stack.push(screen);
    }

    pub fn pop_screen(&mut self) {
        if self.screen_stack.len() > 1 {
            self.screen_stack.pop();
        }
    }

    /// フィルタ適用後の AVC 一覧
    pub fn filtered_avc(&self) -> Vec<&AvcEntry> {
        if self.avc_filter.is_empty() {
            self.avc_entries.iter().collect()
        } else {
            let f = self.avc_filter.to_lowercase();
            self.avc_entries
                .iter()
                .filter(|e| {
                    e.process.to_lowercase().contains(&f)
                        || e.target.to_lowercase().contains(&f)
                        || e.perm.to_lowercase().contains(&f)
                })
                .collect()
        }
    }

    pub fn selected_avc(&self) -> Option<&AvcEntry> {
        let list = self.filtered_avc();
        list.get(self.avc_cursor).copied()
    }

    pub fn cursor_up(&mut self) {
        if self.avc_cursor > 0 {
            self.avc_cursor -= 1;
        }
    }

    /// エントリを処理済みにしてキーを永続セットに登録する
    pub fn mark_resolved(&mut self, id: usize) {
        if let Some(e) = self.avc_entries.iter_mut().find(|e| e.id == id) {
            e.resolved = true;
            let key = format!("{}|{}|{}|{}", e.scontext, e.tcontext, e.tclass, e.perm);
            self.resolved_keys.insert(key);
        }
    }

    /// AVC エントリ一覧を更新し、resolved_keys に一致するエントリの処理済み状態を復元する
    pub fn update_avc_entries(&mut self, mut entries: Vec<AvcEntry>) {
        for e in &mut entries {
            let key = format!("{}|{}|{}|{}", e.scontext, e.tcontext, e.tclass, e.perm);
            if self.resolved_keys.contains(&key) {
                e.resolved = true;
            }
        }
        self.avc_entries = entries;
    }

    pub fn cursor_down(&mut self) {
        let max = self.filtered_avc().len().saturating_sub(1);
        if self.avc_cursor < max {
            self.avc_cursor += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use crate::selinux::avc::Remedy;

    #[test]
    fn test_screen_stack() {
        let mut app = App::new();
        assert_eq!(app.current_screen(), &Screen::AvcList);

        app.push_screen(Screen::ModuleList);
        assert_eq!(app.current_screen(), &Screen::ModuleList);

        app.pop_screen();
        assert_eq!(app.current_screen(), &Screen::AvcList);
    }

    #[test]
    fn test_avc_filtering() {
        let mut app = App::new();
        let e1 = AvcEntry {
            id: 1, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "nginx".to_string(), perm: "read".to_string(), tclass: "file".to_string(),
            scontext: "".to_string(), tcontext: "".to_string(), target: "/var/www/index.html".to_string(),
            raw_lines: vec![], remedy: Remedy::Restorecon, resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };
        let e2 = AvcEntry {
            id: 2, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "httpd".to_string(), perm: "write".to_string(), tclass: "file".to_string(),
            scontext: "".to_string(), tcontext: "".to_string(), target: "/var/log/httpd.log".to_string(),
            raw_lines: vec![], remedy: Remedy::Restorecon, resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };
        app.avc_entries = vec![e1, e2];

        // フィルタなし
        assert_eq!(app.filtered_avc().len(), 2);

        // プロセス名でフィルタ
        app.avc_filter = "nginx".to_string();
        let filtered = app.filtered_avc();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].process, "nginx");

        // ターゲットパスでフィルタ
        app.avc_filter = "/var/log".to_string();
        let filtered = app.filtered_avc();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].process, "httpd");
    }

    #[test]
    fn test_mark_resolved_and_update() {
        let mut app = App::new();
        let e1 = AvcEntry {
            id: 1, first_seen: Local::now(), last_seen: Local::now(), count: 1,
            process: "nginx".into(), perm: "read".into(), tclass: "file".into(),
            scontext: "s:r:nginx_t:s0".into(), tcontext: "s:o:default_t:s0".into(), target: "/a".into(),
            raw_lines: vec![], remedy: Remedy::Restorecon, resolved: false,
            bool_description: None, syscall_name: None, errno_name: None, override_path: None,
        };
        app.avc_entries = vec![e1.clone()];

        // 解決済みにマーク
        app.mark_resolved(1);
        assert!(app.avc_entries[0].resolved);
        assert!(app.resolved_keys.contains("s:r:nginx_t:s0|s:o:default_t:s0|file|read"));

        // 新しいエントリ一覧で更新（再取得をシミュレート）
        let e1_new = AvcEntry { id: 10, resolved: false, ..e1 }; // ID が変わっても OK
        app.update_avc_entries(vec![e1_new]);

        // 解決済み状態が復元されていること
        assert_eq!(app.avc_entries.len(), 1);
        assert!(app.avc_entries[0].resolved);
    }
}
