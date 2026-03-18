/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

/// 言語テーブル定義マクロ
/// 新言語追加時はここの1行だけ変更すれば、mod/enum/detect/impl Lang が全て自動生成される。
///
/// 書式: VariantName => module @ ["locale_prefix", ...]
///   - プレフィックスは上から順に試すため、長いものを先に書く（例: zh_tw より前に zh）
///   - default で指定した言語はロケール不一致時のフォールバックになる
macro_rules! define_langs {
    ( default $default:ident; $( $variant:ident => $module:ident @ [$($prefix:literal),+] ),* $(,)? ) => {

        $( mod $module; )*

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Lang { $( $variant, )* }

        pub fn detect_lang() -> Lang {
            let lang = std::env::var("LANG")
                .or_else(|_| std::env::var("LC_ALL"))
                .or_else(|_| std::env::var("LC_MESSAGES"))
                .unwrap_or_default()
                .to_lowercase();
            $( $( if lang.starts_with($prefix) { return Lang::$variant; } )+ )*
            Lang::$default
        }

        impl Lang {
            // ── フッターヒント ────────────────────────────────────────────────
            pub fn hint_avc_list(&self)      -> &'static str { match self { $( Lang::$variant => $module::HINT_AVC_LIST, )* } }
            pub fn hint_avc_detail(&self)    -> &'static str { match self { $( Lang::$variant => $module::HINT_AVC_DETAIL, )* } }
            pub fn hint_policy_review(&self) -> &'static str { match self { $( Lang::$variant => $module::HINT_POLICY_REVIEW, )* } }
            pub fn hint_auth(&self)          -> &'static str { match self { $( Lang::$variant => $module::HINT_AUTH, )* } }

            // ── テーブルヘッダー ──────────────────────────────────────────────
            pub fn col_occurred(&self) -> &'static str { match self { $( Lang::$variant => $module::COL_OCCURRED, )* } }
            pub fn col_process(&self)  -> &'static str { match self { $( Lang::$variant => $module::COL_PROCESS, )* } }
            pub fn col_action(&self)   -> &'static str { match self { $( Lang::$variant => $module::COL_ACTION, )* } }
            pub fn col_target(&self)   -> &'static str { match self { $( Lang::$variant => $module::COL_TARGET, )* } }
            pub fn col_count(&self)    -> &'static str { match self { $( Lang::$variant => $module::COL_COUNT, )* } }
            pub fn col_remedy(&self)   -> &'static str { match self { $( Lang::$variant => $module::COL_REMEDY, )* } }

            // ── ステータス・メッセージ ────────────────────────────────────────
            pub fn loading_msg(&self)      -> &'static str { match self { $( Lang::$variant => $module::LOADING_MSG, )* } }
            pub fn selinux_disabled(&self) -> &'static str { match self { $( Lang::$variant => $module::SELINUX_DISABLED, )* } }
            pub fn no_avc(&self)           -> &'static str { match self { $( Lang::$variant => $module::NO_AVC, )* } }
            pub fn op_complete(&self)      -> &'static str { match self { $( Lang::$variant => $module::OP_COMPLETE, )* } }
            pub fn ignored(&self)          -> &'static str { match self { $( Lang::$variant => $module::IGNORED, )* } }
            pub fn filter_label(&self)     -> &'static str { match self { $( Lang::$variant => $module::FILTER_LABEL, )* } }

            // ── 認証ポップアップ ──────────────────────────────────────────────
            pub fn auth_title(&self)      -> &'static str { match self { $( Lang::$variant => $module::AUTH_TITLE, )* } }
            pub fn auth_cmd_label(&self)  -> &'static str { match self { $( Lang::$variant => $module::AUTH_CMD_LABEL, )* } }
            pub fn auth_pw_label(&self)   -> &'static str { match self { $( Lang::$variant => $module::AUTH_PW_LABEL, )* } }
            pub fn auth_cancel_btn(&self) -> &'static str { match self { $( Lang::$variant => $module::AUTH_CANCEL_BTN, )* } }
            pub fn auth_exec_btn(&self)   -> &'static str { match self { $( Lang::$variant => $module::AUTH_EXEC_BTN, )* } }
            pub fn pw_wrong(&self)        -> &'static str { match self { $( Lang::$variant => $module::PW_WRONG, )* } }

            // ── 詳細画面ブロック ──────────────────────────────────────────────
            pub fn block_analysis(&self) -> &'static str { match self { $( Lang::$variant => $module::BLOCK_ANALYSIS, )* } }
            pub fn block_options(&self)  -> &'static str { match self { $( Lang::$variant => $module::BLOCK_OPTIONS, )* } }
            pub fn block_raw_log(&self)  -> &'static str { match self { $( Lang::$variant => $module::BLOCK_RAW_LOG, )* } }

            // ── ポリシーレビュー ──────────────────────────────────────────────
            pub fn policy_review_title(&self) -> &'static str { match self { $( Lang::$variant => $module::POLICY_REVIEW_TITLE, )* } }
            pub fn policy_apply_desc(&self)   -> &'static str { match self { $( Lang::$variant => $module::POLICY_APPLY_DESC, )* } }

            // ── 対処オプション（静的） ────────────────────────────────────────
            pub fn opt_restorecon_desc(&self)     -> &'static str { match self { $( Lang::$variant => $module::OPT_RESTORECON_DESC, )* } }
            pub fn opt_custom_policy_label(&self) -> &'static str { match self { $( Lang::$variant => $module::OPT_CUSTOM_POLICY_LABEL, )* } }
            pub fn opt_custom_policy_desc(&self)  -> &'static str { match self { $( Lang::$variant => $module::OPT_CUSTOM_POLICY_DESC, )* } }
            pub fn opt_permissive_desc(&self)     -> &'static str { match self { $( Lang::$variant => $module::OPT_PERMISSIVE_DESC, )* } }
            pub fn opt_ignore_label(&self)        -> &'static str { match self { $( Lang::$variant => $module::OPT_IGNORE_LABEL, )* } }
            pub fn opt_ignore_desc(&self)         -> &'static str { match self { $( Lang::$variant => $module::OPT_IGNORE_DESC, )* } }

            // ── 原因分析（静的） ──────────────────────────────────────────────
            pub fn analysis_fcontext_nonstandard(&self) -> &'static str { match self { $( Lang::$variant => $module::ANALYSIS_FCONTEXT_NONSTANDARD, )* } }
            pub fn analysis_restorecon_fix(&self)       -> &'static str { match self { $( Lang::$variant => $module::ANALYSIS_RESTORECON_FIX, )* } }
            pub fn analysis_custompolicy_fix(&self)     -> &'static str { match self { $( Lang::$variant => $module::ANALYSIS_CUSTOMPOLICY_FIX, )* } }

            // ── Remedy 表示名 ─────────────────────────────────────────────────
            pub fn remedy_port_context(&self)  -> &'static str { match self { $( Lang::$variant => $module::REMEDY_PORT_CONTEXT, )* } }
            pub fn remedy_file_context(&self)  -> &'static str { match self { $( Lang::$variant => $module::REMEDY_FILE_CONTEXT, )* } }
            pub fn remedy_restorecon(&self)    -> &'static str { match self { $( Lang::$variant => $module::REMEDY_RESTORECON, )* } }
            pub fn remedy_custom_policy(&self) -> &'static str { match self { $( Lang::$variant => $module::REMEDY_CUSTOM_POLICY, )* } }

            // ── フォーマット文字列 ────────────────────────────────────────────
            pub fn avc_list_title(&self, unresolved: usize, total: usize) -> String {
                match self { $( Lang::$variant => $module::avc_list_title(unresolved, total), )* }
            }
            pub fn avc_loaded(&self, count: usize) -> String {
                match self { $( Lang::$variant => $module::avc_loaded(count), )* }
            }
            pub fn cmd_failed(&self, first_line: &str) -> String {
                match self { $( Lang::$variant => $module::cmd_failed(first_line), )* }
            }
            pub fn lockout_msg(&self, secs: u64) -> String {
                match self { $( Lang::$variant => $module::lockout_msg(secs), )* }
            }
            pub fn log_overlay_title(&self, total: usize) -> String {
                match self { $( Lang::$variant => $module::log_overlay_title(total), )* }
            }
            pub fn opt_port_label(&self, proto: &str, port: &str) -> String {
                match self { $( Lang::$variant => $module::opt_port_label(proto, port), )* }
            }
            pub fn opt_port_desc(&self, proto: &str, target: &str) -> String {
                match self { $( Lang::$variant => $module::opt_port_desc(proto, target), )* }
            }
            pub fn opt_restorecon_label(&self, path: &str) -> String {
                match self { $( Lang::$variant => $module::opt_restorecon_label(path), )* }
            }
            pub fn opt_fcontext_label(&self, file_type: &str, path: &str) -> String {
                match self { $( Lang::$variant => $module::opt_fcontext_label(file_type, path), )* }
            }
            pub fn opt_fcontext_desc(&self, file_type: &str) -> String {
                match self { $( Lang::$variant => $module::opt_fcontext_desc(file_type), )* }
            }
            pub fn opt_bool_temp_label(&self, bool_name: &str) -> String {
                match self { $( Lang::$variant => $module::opt_bool_temp_label(bool_name), )* }
            }
            pub fn opt_bool_temp_desc(&self, bool_name: &str) -> String {
                match self { $( Lang::$variant => $module::opt_bool_temp_desc(bool_name), )* }
            }
            pub fn opt_bool_perm_label(&self, bool_name: &str) -> String {
                match self { $( Lang::$variant => $module::opt_bool_perm_label(bool_name), )* }
            }
            pub fn opt_bool_perm_desc(&self, bool_name: &str) -> String {
                match self { $( Lang::$variant => $module::opt_bool_perm_desc(bool_name), )* }
            }
            pub fn opt_permissive_label(&self, domain: &str) -> String {
                match self { $( Lang::$variant => $module::opt_permissive_label(domain), )* }
            }
            pub fn analysis_denied(&self, process: &str, target: &str, perm: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_denied(process, target, perm), )* }
            }
            pub fn analysis_port_undefined(&self, target: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_port_undefined(target), )* }
            }
            pub fn analysis_port_nonstandard(&self, process: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_port_nonstandard(process), )* }
            }
            pub fn analysis_write_denied(&self, target: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_write_denied(target), )* }
            }
            pub fn analysis_label_stripped(&self, target: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_label_stripped(target), )* }
            }
            pub fn analysis_bool_enable(&self, b: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_bool_enable(b), )* }
            }
            pub fn analysis_domain_denied(&self, domain: &str, perm: &str) -> String {
                match self { $( Lang::$variant => $module::analysis_domain_denied(domain, perm), )* }
            }
            pub fn remedy_boolean(&self, b: &str) -> String {
                match self { $( Lang::$variant => $module::remedy_boolean(b), )* }
            }
            pub fn elapsed_secs(&self, n: u64)  -> String { match self { $( Lang::$variant => $module::elapsed_secs(n), )* } }
            pub fn elapsed_mins(&self, n: u64)  -> String { match self { $( Lang::$variant => $module::elapsed_mins(n), )* } }
            pub fn elapsed_hours(&self, n: u64) -> String { match self { $( Lang::$variant => $module::elapsed_hours(n), )* } }
            pub fn elapsed_days(&self, n: u64)  -> String { match self { $( Lang::$variant => $module::elapsed_days(n), )* } }
            pub fn warn_locale_not_utf8(&self, lang_val: &str) -> String {
                match self { $( Lang::$variant => $module::warn_locale_not_utf8(lang_val), )* }
            }
        }
    };
}

// ── 言語テーブル ──────────────────────────────────────────────────────────────
// 新言語を追加する場合はここに1行追加するだけ。
// プレフィックスが重複する場合は上の行が優先されるため、長い（具体的な）ものを先に書く。
define_langs! {
    default English;
    Japanese           => ja      @ ["ja"],
    ChineseTraditional => zh_hant @ ["zh_tw", "zh_hk", "zh_mo"],
    Chinese            => zh_hans @ ["zh"],
    Korean             => ko      @ ["ko"],
    Russian            => ru      @ ["ru"],
    Kazakh             => kk      @ ["kk"],
    Spanish            => es      @ ["es"],
    Portuguese         => pt      @ ["pt"],
    French             => fr      @ ["fr"],
    German             => de      @ ["de"],
    Italian            => it      @ ["it"],
    Dutch              => nl      @ ["nl"],
    Swedish            => sv      @ ["sv"],
    Norwegian          => no      @ ["nb", "nn", "no"],
    Arabic             => ar      @ ["ar"],
    Thai               => th      @ ["th"],
    Vietnamese         => vi      @ ["vi"],
    English            => en      @ ["en"],
}

// ── ローディングスピナー ──────────────────────────────────────────────────────
impl Lang {
    /// tick に応じたスピナーフレームとラベルを組み合わせた文字列を返す。
    /// label が None のときは loading_msg() を使用する。
    pub fn loading_spinner(&self, tick: u64, label: Option<&str>) -> String {
        const FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let frame = FRAMES[(tick as usize) % FRAMES.len()];
        let text = label.unwrap_or_else(|| self.loading_msg());
        format!("{} {}", frame, text)
    }
}

// ── ロケール別日時書式 ────────────────────────────────────────────────────────
// chrono の strftime 書式文字列を返す。
impl Lang {
    pub fn datetime_format(&self) -> &'static str {
        match self {
            // 漢字圏
            Lang::Japanese | Lang::Chinese | Lang::ChineseTraditional => "%Y年%m月%d日 %H:%M",
            // 韓国語
            Lang::Korean => "%Y년 %m월 %d일 %H:%M",
            // ドット区切り (DD.MM.YYYY)
            Lang::Russian | Lang::Kazakh | Lang::German | Lang::Norwegian => "%d.%m.%Y %H:%M",
            // スラッシュ区切り (DD/MM/YYYY)
            Lang::Spanish
            | Lang::Portuguese
            | Lang::French
            | Lang::Italian
            | Lang::Arabic
            | Lang::Thai
            | Lang::Vietnamese => "%d/%m/%Y %H:%M",
            // ダッシュ区切り (DD-MM-YYYY)
            Lang::Dutch => "%d-%m-%Y %H:%M",
            // ISO 順 (YYYY-MM-DD) — Swedish・英語
            Lang::Swedish | Lang::English => "%Y-%m-%d %H:%M",
        }
    }
}
