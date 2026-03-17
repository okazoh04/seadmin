use crate::selinux::avc::AvcEntry;
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
    pub auth_error: Option<String>,
    pub status_message: Option<String>,
    pub loading: bool,
    /// PolicyReview 画面のスクロール位置
    pub policy_review_scroll: usize,
    /// 操作ログ（最大 500 件、新しいものが末尾）
    pub log: VecDeque<String>,
    pub log_scroll: usize,
    pub show_log: bool,
    /// ログファイル（~/.local/share/seadmin/seadmin.log）
    pub log_file: Option<std::fs::File>,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen_stack: vec![Screen::AvcList],
            avc_entries: Vec::new(),
            avc_cursor: 0,
            avc_filter: String::new(),
            avc_filter_active: false,
            selinux_mode: "UNKNOWN".to_string(),
            hostname: String::new(),
            auth_state: AuthState::default(),
            password_buf: Zeroizing::new(String::new()),
            cached_password: None,
            auth_error: None,
            status_message: None,
            loading: false,
            policy_review_scroll: 0,
            log: VecDeque::new(),
            log_scroll: 0,
            show_log: false,
            log_file: None,
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

    pub fn cursor_down(&mut self) {
        let max = self.filtered_avc().len().saturating_sub(1);
        if self.avc_cursor < max {
            self.avc_cursor += 1;
        }
    }
}
