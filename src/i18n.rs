/// 表示言語
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    Japanese,
    English,
    Chinese,
}

/// LANG / LC_ALL / LC_MESSAGES 環境変数からロケールを判定する
pub fn detect_lang() -> Lang {
    let lang = std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .unwrap_or_default()
        .to_lowercase();

    if lang.starts_with("zh") {
        Lang::Chinese
    } else if lang.starts_with("ja") {
        Lang::Japanese
    } else {
        Lang::English
    }
}

impl Lang {
    // ── フッターヒント ────────────────────────────────────────────────

    pub fn hint_avc_list(&self) -> &'static str {
        match self {
            Lang::Japanese => "↑↓/jk:移動  Enter:詳細  /:フィルタ  r:更新  l:ログ  q:終了",
            Lang::English  => "↑↓/jk:Move  Enter:Detail  /:Filter  r:Reload  l:Log  q:Quit",
            Lang::Chinese  => "↑↓/jk:移动  Enter:详情  /:过滤  r:刷新  l:日志  q:退出",
        }
    }

    pub fn hint_avc_detail(&self) -> &'static str {
        match self {
            Lang::Japanese => "A-F:対処選択  Esc/←:戻る  Enter:確認へ",
            Lang::English  => "A-F:Select  Esc/←:Back  Enter:Confirm",
            Lang::Chinese  => "A-F:选择处置  Esc/←:返回  Enter:确认",
        }
    }

    pub fn hint_policy_review(&self) -> &'static str {
        match self {
            Lang::Japanese => "↑↓/jk:スクロール  Enter:適用  Esc:キャンセル",
            Lang::English  => "↑↓/jk:Scroll  Enter:Apply  Esc:Cancel",
            Lang::Chinese  => "↑↓/jk:滚动  Enter:应用  Esc:取消",
        }
    }

    pub fn hint_auth(&self) -> &'static str {
        match self {
            Lang::Japanese => "Enter:実行  Esc:キャンセル",
            Lang::English  => "Enter:Execute  Esc:Cancel",
            Lang::Chinese  => "Enter:执行  Esc:取消",
        }
    }

    // ── テーブルヘッダー ─────────────────────────────────────────────

    pub fn col_occurred(&self) -> &'static str {
        match self {
            Lang::Japanese => "発生",
            Lang::English  => "When",
            Lang::Chinese  => "时间",
        }
    }

    pub fn col_process(&self) -> &'static str {
        match self {
            Lang::Japanese => "プロセス",
            Lang::English  => "Process",
            Lang::Chinese  => "进程",
        }
    }

    pub fn col_action(&self) -> &'static str {
        match self {
            Lang::Japanese => "操作",
            Lang::English  => "Action",
            Lang::Chinese  => "操作",
        }
    }

    pub fn col_target(&self) -> &'static str {
        match self {
            Lang::Japanese => "対象",
            Lang::English  => "Target",
            Lang::Chinese  => "目标",
        }
    }

    pub fn col_count(&self) -> &'static str {
        match self {
            Lang::Japanese => "件数",
            Lang::English  => "Count",
            Lang::Chinese  => "次数",
        }
    }

    pub fn col_remedy(&self) -> &'static str {
        match self {
            Lang::Japanese => "解決策候補",
            Lang::English  => "Remedy",
            Lang::Chinese  => "修复方案",
        }
    }

    // ── AVC 一覧タイトル ─────────────────────────────────────────────

    pub fn avc_list_title(&self, unresolved: usize, total: usize) -> String {
        match self {
            Lang::Japanese => format!(" アクセス拒否一覧  [本日]  未対処: {}件 / 全 {}件 ", unresolved, total),
            Lang::English  => format!(" Access Denials  [Today]  Unresolved: {} / Total: {} ", unresolved, total),
            Lang::Chinese  => format!(" 访问拒绝列表  [今日]  未处理: {}条 / 共 {}条 ", unresolved, total),
        }
    }

    // ── フィルタ ─────────────────────────────────────────────────────

    pub fn filter_label(&self) -> &'static str {
        match self {
            Lang::Japanese => "/フィルタ: ",
            Lang::English  => "/Filter: ",
            Lang::Chinese  => "/过滤: ",
        }
    }

    // ── ローディング ─────────────────────────────────────────────────

    pub fn loading_msg(&self) -> &'static str {
        match self {
            Lang::Japanese => " ⏳ AVC ログを読み込み中...",
            Lang::English  => " ⏳ Loading AVC log...",
            Lang::Chinese  => " ⏳ 正在加载 AVC 日志...",
        }
    }

    // ── ステータスメッセージ ─────────────────────────────────────────

    pub fn selinux_disabled(&self) -> &'static str {
        match self {
            Lang::Japanese => "⚠ SELinux が無効です。アクセス拒否は記録されません。",
            Lang::English  => "⚠ SELinux is disabled. Access denials will not be recorded.",
            Lang::Chinese  => "⚠ SELinux 已禁用。访问拒绝事件将不会被记录。",
        }
    }

    pub fn no_avc(&self) -> &'static str {
        match self {
            Lang::Japanese => "アクセス拒否はありません",
            Lang::English  => "No access denials",
            Lang::Chinese  => "没有访问拒绝事件",
        }
    }

    pub fn avc_loaded(&self, count: usize) -> String {
        match self {
            Lang::Japanese => format!("{} 件の AVC を取得しました", count),
            Lang::English  => format!("Loaded {} AVC entries", count),
            Lang::Chinese  => format!("已加载 {} 个 AVC 条目", count),
        }
    }

    pub fn op_complete(&self) -> &'static str {
        match self {
            Lang::Japanese => "操作が完了しました",
            Lang::English  => "Operation completed",
            Lang::Chinese  => "操作已完成",
        }
    }

    pub fn ignored(&self) -> &'static str {
        match self {
            Lang::Japanese => "無視リストに追加しました",
            Lang::English  => "Added to ignore list",
            Lang::Chinese  => "已添加到忽略列表",
        }
    }

    pub fn cmd_failed(&self, first_line: &str) -> String {
        match self {
            Lang::Japanese => format!("コマンド失敗: {}", first_line),
            Lang::English  => format!("Command failed: {}", first_line),
            Lang::Chinese  => format!("命令失败: {}", first_line),
        }
    }

    // ── 認証ポップアップ ─────────────────────────────────────────────

    pub fn auth_title(&self) -> &'static str {
        match self {
            Lang::Japanese => "🔒 管理者認証",
            Lang::English  => "🔒 Administrator Authentication",
            Lang::Chinese  => "🔒 管理员认证",
        }
    }

    pub fn auth_cmd_label(&self) -> &'static str {
        match self {
            Lang::Japanese => "  実行コマンド：",
            Lang::English  => "  Command:",
            Lang::Chinese  => "  执行命令：",
        }
    }

    pub fn auth_pw_label(&self) -> &'static str {
        match self {
            Lang::Japanese => "  パスワード：",
            Lang::English  => "  Password:",
            Lang::Chinese  => "  密码：",
        }
    }

    pub fn auth_cancel_btn(&self) -> &'static str {
        match self {
            Lang::Japanese => "[ キャンセル（Esc） ]",
            Lang::English  => "[ Cancel (Esc) ]",
            Lang::Chinese  => "[ 取消（Esc） ]",
        }
    }

    pub fn auth_exec_btn(&self) -> &'static str {
        match self {
            Lang::Japanese => "[ 実行（Enter） ]",
            Lang::English  => "[ Execute (Enter) ]",
            Lang::Chinese  => "[ 执行（Enter） ]",
        }
    }

    pub fn pw_wrong(&self) -> &'static str {
        match self {
            Lang::Japanese => "パスワードが正しくありません",
            Lang::English  => "Incorrect password",
            Lang::Chinese  => "密码不正确",
        }
    }

    pub fn lockout_msg(&self, secs: u64) -> String {
        match self {
            Lang::Japanese => format!("  ロックアウト中（{}秒後に解除）", secs),
            Lang::English  => format!("  Locked out ({} seconds remaining)", secs),
            Lang::Chinese  => format!("  已锁定（{}秒后解除）", secs),
        }
    }

    // ── 詳細画面ブロックタイトル ─────────────────────────────────────

    pub fn block_analysis(&self) -> &'static str {
        match self {
            Lang::Japanese => " 原因分析 ",
            Lang::English  => " Analysis ",
            Lang::Chinese  => " 原因分析 ",
        }
    }

    pub fn block_options(&self) -> &'static str {
        match self {
            Lang::Japanese => " 対処オプション ",
            Lang::English  => " Remediation Options ",
            Lang::Chinese  => " 处置选项 ",
        }
    }

    pub fn block_raw_log(&self) -> &'static str {
        match self {
            Lang::Japanese => " 生ログ（参照用）",
            Lang::English  => " Raw Log (reference)",
            Lang::Chinese  => " 原始日志（参考）",
        }
    }

    // ── ポリシーレビュー ─────────────────────────────────────────────

    pub fn policy_review_title(&self) -> &'static str {
        match self {
            Lang::Japanese => " ポリシー内容確認（Enter:適用  Esc:キャンセル）",
            Lang::English  => " Policy Review (Enter:Apply  Esc:Cancel)",
            Lang::Chinese  => " 策略内容确认（Enter:应用  Esc:取消）",
        }
    }

    pub fn policy_apply_desc(&self) -> &'static str {
        match self {
            Lang::Japanese => "生成したポリシーモジュールをシステムに適用します。",
            Lang::English  => "Apply the generated policy module to the system.",
            Lang::Chinese  => "将生成的策略模块应用到系统。",
        }
    }

    // ── ログオーバーレイ ─────────────────────────────────────────────

    pub fn log_overlay_title(&self, total: usize) -> String {
        match self {
            Lang::Japanese => format!(" ログ  {} 件  ↑↓:スクロール  l:閉じる ", total),
            Lang::English  => format!(" Log  {} entries  ↑↓:Scroll  l:Close ", total),
            Lang::Chinese  => format!(" 日志  {} 条  ↑↓:滚动  l:关闭 ", total),
        }
    }

    // ── 対処オプション文字列 ─────────────────────────────────────────

    pub fn opt_port_label(&self, proto: &str, port: &str) -> String {
        match self {
            Lang::Japanese => format!("ポートコンテキストを追加  semanage port -a -t ssh_port_t -p {} {}", proto, port),
            Lang::English  => format!("Add port context  semanage port -a -t ssh_port_t -p {} {}", proto, port),
            Lang::Chinese  => format!("添加端口上下文  semanage port -a -t ssh_port_t -p {} {}", proto, port),
        }
    }

    pub fn opt_port_desc(&self, proto: &str, target: &str) -> String {
        match self {
            Lang::Japanese => format!("{} の {} ポートに ssh_port_t コンテキストを付与します。", proto, target),
            Lang::English  => format!("Assign ssh_port_t context to {} port {}.", proto, target),
            Lang::Chinese  => format!("为 {} 的 {} 端口分配 ssh_port_t 上下文。", proto, target),
        }
    }

    pub fn opt_restorecon_label(&self, path: &str) -> String {
        match self {
            Lang::Japanese => format!("restorecon で修復  restorecon -Rv {}", path),
            Lang::English  => format!("Repair with restorecon  restorecon -Rv {}", path),
            Lang::Chinese  => format!("使用 restorecon 修复  restorecon -Rv {}", path),
        }
    }

    pub fn opt_restorecon_desc(&self) -> &'static str {
        match self {
            Lang::Japanese => "デフォルトのファイルコンテキストに戻します（ラベル剥がれの修復）。",
            Lang::English  => "Restore the default file context (repair stripped labels).",
            Lang::Chinese  => "恢复默认文件上下文（修复标签丢失问题）。",
        }
    }

    pub fn opt_fcontext_label(&self, file_type: &str, path: &str) -> String {
        match self {
            Lang::Japanese => format!("fcontext を変更  semanage fcontext -a -t {} {}(.*)", file_type, path),
            Lang::English  => format!("Change fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path),
            Lang::Chinese  => format!("更改 fcontext  semanage fcontext -a -t {} {}(.*)", file_type, path),
        }
    }

    pub fn opt_fcontext_desc(&self, file_type: &str) -> String {
        match self {
            Lang::Japanese => format!("このパスに {} を付与するルールを追加します。適用後 restorecon も実行してください。", file_type),
            Lang::English  => format!("Add a rule to assign {} to this path. Run restorecon after applying.", file_type),
            Lang::Chinese  => format!("添加将 {} 分配给此路径的规则。应用后请同时运行 restorecon。", file_type),
        }
    }

    pub fn opt_bool_temp_label(&self, bool_name: &str) -> String {
        match self {
            Lang::Japanese => format!("Boolean を有効化（一時）  setsebool {} on", bool_name),
            Lang::English  => format!("Enable Boolean (temporary)  setsebool {} on", bool_name),
            Lang::Chinese  => format!("启用 Boolean（临时）  setsebool {} on", bool_name),
        }
    }

    pub fn opt_bool_temp_desc(&self, bool_name: &str) -> String {
        match self {
            Lang::Japanese => format!("{} を有効にします（再起動で元に戻ります）。", bool_name),
            Lang::English  => format!("Enable {} (reverts after reboot).", bool_name),
            Lang::Chinese  => format!("启用 {}（重启后恢复）。", bool_name),
        }
    }

    pub fn opt_bool_perm_label(&self, bool_name: &str) -> String {
        match self {
            Lang::Japanese => format!("Boolean を有効化（永続）  setsebool -P {} on", bool_name),
            Lang::English  => format!("Enable Boolean (persistent)  setsebool -P {} on", bool_name),
            Lang::Chinese  => format!("启用 Boolean（持久）  setsebool -P {} on", bool_name),
        }
    }

    pub fn opt_bool_perm_desc(&self, bool_name: &str) -> String {
        match self {
            Lang::Japanese => format!("{} を永続的に有効にします。", bool_name),
            Lang::English  => format!("Persistently enable {}.", bool_name),
            Lang::Chinese  => format!("持久启用 {}。", bool_name),
        }
    }

    pub fn opt_custom_policy_label(&self) -> &'static str {
        match self {
            Lang::Japanese => "カスタムポリシーモジュールを生成・適用（audit2allow）",
            Lang::English  => "Generate and apply custom policy module (audit2allow)",
            Lang::Chinese  => "生成并应用自定义策略模块（audit2allow）",
        }
    }

    pub fn opt_custom_policy_desc(&self) -> &'static str {
        match self {
            Lang::Japanese => "audit2allow でポリシーを自動生成します。内容をレビューしてから適用できます。",
            Lang::English  => "Auto-generate a policy with audit2allow. Review before applying.",
            Lang::Chinese  => "使用 audit2allow 自动生成策略。可在应用前进行审核。",
        }
    }

    pub fn opt_permissive_label(&self, domain: &str) -> String {
        match self {
            Lang::Japanese => format!("このドメインを Permissive に設定（調査用）⚠  semanage permissive -a {}", domain),
            Lang::English  => format!("Set domain to Permissive (investigation only) ⚠  semanage permissive -a {}", domain),
            Lang::Chinese  => format!("将域设为 Permissive（仅用于调查）⚠  semanage permissive -a {}", domain),
        }
    }

    pub fn opt_permissive_desc(&self) -> &'static str {
        match self {
            Lang::Japanese => "拒否を一時的に無効化します。セキュリティが低下するため調査目的に限定してください。",
            Lang::English  => "Temporarily disable denials. Reduces security; use only for investigation.",
            Lang::Chinese  => "临时禁用拒绝。会降低安全性，请仅用于调查目的。",
        }
    }

    pub fn opt_ignore_label(&self) -> &'static str {
        match self {
            Lang::Japanese => "何もしない / 無視リストに追加",
            Lang::English  => "Do nothing / Add to ignore list",
            Lang::Chinese  => "不处理 / 添加到忽略列表",
        }
    }

    pub fn opt_ignore_desc(&self) -> &'static str {
        match self {
            Lang::Japanese => "このエントリを無視リストに追加します（ツール内のみ）。",
            Lang::English  => "Add this entry to the ignore list (tool-local only).",
            Lang::Chinese  => "将此条目添加到忽略列表（仅限工具内部）。",
        }
    }

    // ── 原因分析テキスト ─────────────────────────────────────────────

    pub fn analysis_denied(&self, process: &str, target: &str, perm: &str) -> String {
        match self {
            Lang::Japanese => format!(" {} が {} への {} を拒否されました。", process, target, perm),
            Lang::English  => format!(" {} was denied {} on {}.", process, perm, target),
            Lang::Chinese  => format!(" {} 对 {} 的 {} 操作被拒绝。", process, target, perm),
        }
    }

    pub fn analysis_port_undefined(&self, target: &str) -> String {
        match self {
            Lang::Japanese => format!(" ポート {} は SELinux ポリシー上で未定義です。", target),
            Lang::English  => format!(" Port {} is not defined in the SELinux policy.", target),
            Lang::Chinese  => format!(" 端口 {} 在 SELinux 策略中未定义。", target),
        }
    }

    pub fn analysis_port_nonstandard(&self, process: &str) -> String {
        match self {
            Lang::Japanese => format!(" {} を非標準ポートで動作させるにはポートコンテキストの追加が必要です。", process),
            Lang::English  => format!(" {} needs a port context added to operate on a non-standard port.", process),
            Lang::Chinese  => format!(" {} 在非标准端口运行需要添加端口上下文。", process),
        }
    }

    pub fn analysis_write_denied(&self, target: &str) -> String {
        match self {
            Lang::Japanese => format!(" {} への書き込みが拒否されました。", target),
            Lang::English  => format!(" Write access to {} was denied.", target),
            Lang::Chinese  => format!(" 对 {} 的写入被拒绝。", target),
        }
    }

    pub fn analysis_fcontext_nonstandard(&self) -> &'static str {
        match self {
            Lang::Japanese => " 非標準パスのため fcontext ルールの追加が必要です。",
            Lang::English  => " Non-standard path requires adding an fcontext rule.",
            Lang::Chinese  => " 非标准路径需要添加 fcontext 规则。",
        }
    }

    pub fn analysis_label_stripped(&self, target: &str) -> String {
        match self {
            Lang::Japanese => format!(" {} のラベルが剥がれている可能性があります。", target),
            Lang::English  => format!(" The label on {} may have been stripped.", target),
            Lang::Chinese  => format!(" {} 的标签可能已丢失。", target),
        }
    }

    pub fn analysis_restorecon_fix(&self) -> &'static str {
        match self {
            Lang::Japanese => " restorecon でデフォルトコンテキストに戻すことで解決できます。",
            Lang::English  => " Running restorecon to restore the default context may resolve this.",
            Lang::Chinese  => " 运行 restorecon 恢复默认上下文可能解决此问题。",
        }
    }

    pub fn analysis_bool_enable(&self, b: &str) -> String {
        match self {
            Lang::Japanese => format!(" {} Boolean を有効にすることで解決できる可能性があります。", b),
            Lang::English  => format!(" Enabling the {} Boolean may resolve this.", b),
            Lang::Chinese  => format!(" 启用 {} Boolean 可能解决此问题。", b),
        }
    }

    pub fn analysis_domain_denied(&self, domain: &str, perm: &str) -> String {
        match self {
            Lang::Japanese => format!(" ドメイン {} からの {} 操作がポリシーで許可されていません。", domain, perm),
            Lang::English  => format!(" {} operation from domain {} is not allowed by policy.", perm, domain),
            Lang::Chinese  => format!(" 域 {} 的 {} 操作未被策略允许。", domain, perm),
        }
    }

    pub fn analysis_custompolicy_fix(&self) -> &'static str {
        match self {
            Lang::Japanese => " audit2allow でカスタムポリシーを生成する必要があります。",
            Lang::English  => " A custom policy needs to be generated with audit2allow.",
            Lang::Chinese  => " 需要使用 audit2allow 生成自定义策略。",
        }
    }

    // ── Remedy 表示名 ────────────────────────────────────────────────

    pub fn remedy_port_context(&self) -> &'static str {
        match self {
            Lang::Japanese => "ポート追加",
            Lang::English  => "Port Context",
            Lang::Chinese  => "添加端口",
        }
    }

    pub fn remedy_file_context(&self) -> &'static str {
        match self {
            Lang::Japanese => "fcontext変更",
            Lang::English  => "File Context",
            Lang::Chinese  => "fcontext更改",
        }
    }

    pub fn remedy_restorecon(&self) -> &'static str {
        match self {
            Lang::Japanese | Lang::English | Lang::Chinese => "restorecon",
        }
    }

    pub fn remedy_boolean(&self, b: &str) -> String {
        format!("Boolean: {}", b)
    }

    pub fn remedy_custom_policy(&self) -> &'static str {
        match self {
            Lang::Japanese => "カスタムポリシー",
            Lang::English  => "Custom Policy",
            Lang::Chinese  => "自定义策略",
        }
    }

    // ── 相対時刻 ─────────────────────────────────────────────────────

    pub fn elapsed_secs(&self, n: u64) -> String {
        match self {
            Lang::Japanese => format!("{}秒前", n),
            Lang::English  => format!("{}s ago", n),
            Lang::Chinese  => format!("{}秒前", n),
        }
    }

    pub fn elapsed_mins(&self, n: u64) -> String {
        match self {
            Lang::Japanese => format!("{}分前", n),
            Lang::English  => format!("{}m ago", n),
            Lang::Chinese  => format!("{}分前", n),
        }
    }

    pub fn elapsed_hours(&self, n: u64) -> String {
        match self {
            Lang::Japanese => format!("{}時間前", n),
            Lang::English  => format!("{}h ago", n),
            Lang::Chinese  => format!("{}小时前", n),
        }
    }

    pub fn elapsed_days(&self, n: u64) -> String {
        match self {
            Lang::Japanese => format!("{}日前", n),
            Lang::English  => format!("{}d ago", n),
            Lang::Chinese  => format!("{}天前", n),
        }
    }

    // ── 起動時ロケール警告 ───────────────────────────────────────────

    pub fn warn_locale_not_utf8(&self, lang_val: &str) -> String {
        match self {
            Lang::Japanese => format!(
                "警告: ロケールが UTF-8 ではない可能性があります（LANG={}）。\n日本語が正しく表示されない場合は LANG=ja_JP.UTF-8 を設定してください。",
                lang_val
            ),
            Lang::English => format!(
                "Warning: Locale may not be UTF-8 (LANG={}).\nSet LANG=en_US.UTF-8 if characters display incorrectly.",
                lang_val
            ),
            Lang::Chinese => format!(
                "警告: 区域设置可能不是 UTF-8 (LANG={})。\n如果字符显示不正确，请设置 LANG=zh_CN.UTF-8。",
                lang_val
            ),
        }
    }
}
