/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use chrono::{DateTime, Local};
use std::fmt;

use crate::i18n::Lang;

/// アクセス拒否の解決策候補
#[derive(Debug, Clone, PartialEq)]
pub enum Remedy {
    PortContext,
    FileContext,
    Restorecon,
    Boolean(String),
    CustomPolicy,
}

impl fmt::Display for Remedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Remedy::PortContext => write!(f, "PortContext"),
            Remedy::FileContext => write!(f, "FileContext"),
            Remedy::Restorecon => write!(f, "Restorecon"),
            Remedy::Boolean(b) => write!(f, "Boolean({})", b),
            Remedy::CustomPolicy => write!(f, "CustomPolicy"),
        }
    }
}

/// 深刻度レベル
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    /// 緑: restorecon で修復可能
    Low,
    /// 黄: boolean 変更 / ポート追加
    Medium,
    /// 赤: カスタムポリシー生成が必要（未知の拒否）、または制約違反
    High,
}

impl Remedy {
    /// Remedy から深刻度レベルを返す
    pub fn severity(&self) -> Severity {
        match self {
            Remedy::Restorecon => Severity::Low,
            Remedy::Boolean(_) | Remedy::PortContext | Remedy::FileContext => Severity::Medium,
            Remedy::CustomPolicy => Severity::High,
        }
    }

    /// ロケール対応の表示文字列
    pub fn display_str(&self, lang: &Lang) -> String {
        match self {
            Remedy::PortContext  => lang.remedy_port_context().to_string(),
            Remedy::FileContext  => lang.remedy_file_context().to_string(),
            Remedy::Restorecon  => lang.remedy_restorecon().to_string(),
            Remedy::Boolean(b)  => lang.remedy_boolean(b),
            Remedy::CustomPolicy => lang.remedy_custom_policy().to_string(),
        }
    }
}

/// 1件のアクセス拒否（集計済み）
#[derive(Debug, Clone)]
pub struct AvcEntry {
    pub id: usize,
    pub first_seen: DateTime<Local>,
    pub last_seen: DateTime<Local>,
    pub count: usize,
    pub process: String,
    pub perm: String,
    pub tclass: String,
    pub scontext: String,
    pub tcontext: String,
    pub target: String,
    pub raw_lines: Vec<String>,
    pub remedy: Remedy,
    pub resolved: bool,
    /// Boolean remedy 時の sepolicy 説明文（非同期で補完）
    pub bool_description: Option<String>,
    /// SYSCALL レコードから変換したシステムコール名（例: "openat"）
    pub syscall_name: Option<String>,
    /// SYSCALL レコードの exit 値から変換した errno 名（例: "EACCES"）
    pub errno_name: Option<String>,
}

impl AvcEntry {
    /// 相対時刻の文字列表現（ロケール対応）
    pub fn elapsed_str(&self, lang: &Lang) -> String {
        let now = Local::now();
        let secs = (now - self.last_seen).num_seconds().max(0) as u64;
        if secs < 60 {
            lang.elapsed_secs(secs)
        } else if secs < 3600 {
            lang.elapsed_mins(secs / 60)
        } else if secs < 86400 {
            lang.elapsed_hours(secs / 3600)
        } else {
            lang.elapsed_days(secs / 86400)
        }
    }
}

/// `ausearch -m AVC,USER_AVC,SELINUX_ERR -i` の出力を解析して AVC エントリ一覧を返す
pub fn parse_ausearch_output(raw: &str) -> Vec<AvcEntry> {
    // ausearch --interpret 出力は "----" でレコードが区切られる
    let blocks: Vec<&str> = raw.split("----").collect();

    // パス 1: 全ブロックの type=PATH レコードから「ファイル名 → 絶対パス」対応表を構築
    // execute 拒否など一部ブロックは PATH レコードに絶対パスが無いため、
    // 他ブロックで同名ファイルの絶対パスが判明している場合にそれを流用する。
    let path_map = build_path_map(&blocks);

    let mut map: std::collections::HashMap<String, AvcEntry> = std::collections::HashMap::new();
    let mut next_id = 1usize;

    for block in &blocks {
        if let Some(entry) = parse_block(block, next_id, &path_map) {
            // scontext + tcontext + tclass + perm でキー集計
            let key = format!(
                "{}|{}|{}|{}",
                entry.scontext, entry.tcontext, entry.tclass, entry.perm
            );
            match map.get_mut(&key) {
                Some(existing) => {
                    existing.count += 1;
                    if entry.last_seen > existing.last_seen {
                        existing.last_seen = entry.last_seen;
                    }
                    if entry.first_seen < existing.first_seen {
                        existing.first_seen = entry.first_seen;
                    }
                    existing.raw_lines.extend(entry.raw_lines);
                }
                None => {
                    next_id += 1;
                    map.insert(key, entry);
                }
            }
        }
    }

    let mut entries: Vec<AvcEntry> = map.into_values().collect();
    entries.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
    // id を振り直す
    for (i, e) in entries.iter_mut().enumerate() {
        e.id = i + 1;
    }
    entries
}

/// 全ブロックから「ファイル名（basename）→ 絶対パス」対応表を構築する
///
/// 収集元:
///   1. type=PATH レコードの name= フィールド
///   2. AVC 行の path= フィールド（絶対パスが直接含まれる場合）
///
/// perm=execute の AVC 行は name="screen" しか持たないが、
/// 同じファイルに対する perm=map / execute_no_trans の AVC 行には
/// path="/usr/bin/screen" が入っているため、そちらから補完する。
fn build_path_map(blocks: &[&str]) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for block in blocks {
        // 収集元 1: type=PATH レコードの name=
        for line in block.lines().filter(|l| l.contains("type=PATH")) {
            insert_if_absolute(&mut map, extract_field(line, "name="));
        }
        // 収集元 2: AVC 行の path=（他の perm で解決済みの絶対パスを流用）
        for line in block.lines().filter(|l| l.contains("avc:") && l.contains("denied")) {
            insert_if_absolute(&mut map, extract_field(line, "path="));
        }
    }
    map
}

/// 絶対パスであれば basename → abs のエントリをマップに追加する（上書きなし）
fn insert_if_absolute(map: &mut std::collections::HashMap<String, String>, val: Option<String>) {
    if let Some(abs) = val {
        if abs.starts_with('/') {
            let basename = abs.rsplit('/').next().unwrap_or(&abs).to_string();
            map.entry(basename).or_insert(abs);
        }
    }
}

fn parse_block(
    block: &str,
    id: usize,
    path_map: &std::collections::HashMap<String, String>,
) -> Option<AvcEntry> {
    // AVC 行を探す
    let avc_line = block
        .lines()
        .find(|l| l.contains("avc:") && l.contains("denied"))?;

    let process = extract_field(avc_line, "comm=")
        .unwrap_or_else(|| "unknown".to_string())
        .trim_matches('"')
        .to_string();

    // perm: "{ name_bind }" の形式
    let perm = {
        let start = avc_line.find("{ ")?;
        let end = avc_line.find(" }")?;
        avc_line[start + 2..end].trim().to_string()
    };

    let tclass = extract_field(avc_line, "tclass=").unwrap_or_else(|| "unknown".to_string());
    let scontext = extract_field(avc_line, "scontext=").unwrap_or_else(|| "unknown".to_string());
    let tcontext = extract_field(avc_line, "tcontext=").unwrap_or_else(|| "unknown".to_string());

    // 対象リソース
    let target = resolve_target(avc_line, block, &tclass, &tcontext, path_map);

    // タイムスタンプ: ausearch の生出力（-i なし）は "msg=audit(1710000000.123:456)" の epoch 秒形式
    // -i を使うとロケール依存の日付文字列になるため使用しない
    let ts = block
        .lines()
        .find_map(|l| {
            let start = l.find("msg=audit(")?;
            let inner = &l[start + 10..];
            let end = inner.find('.')?;
            let epoch_secs: i64 = inner[..end].parse().ok()?;
            DateTime::from_timestamp(epoch_secs, 0)
                .map(|utc| utc.with_timezone(&Local))
        })
        .unwrap_or_else(Local::now);

    let remedy = diagnose_remedy(&perm, &tclass, &scontext, &tcontext, &target);

    // SYSCALL レコードから syscall 番号と exit コードを抽出して名前変換
    let syscall_line = block.lines().find(|l| l.contains("type=SYSCALL"));
    let syscall_name = syscall_line
        .and_then(|l| extract_field(l, "syscall="))
        .and_then(|s| s.parse::<u32>().ok())
        .and_then(syscall_name_from_nr)
        .map(str::to_string);
    let errno_name = syscall_line
        .and_then(|l| extract_field(l, "exit="))
        .and_then(|s| s.parse::<i32>().ok())
        .filter(|&n| n < 0)
        .and_then(|n| errno_name_from_code((-n) as u32))
        .map(str::to_string);

    Some(AvcEntry {
        id,
        first_seen: ts,
        last_seen: ts,
        count: 1,
        process,
        perm,
        tclass,
        scontext,
        tcontext,
        target,
        raw_lines: block.lines().map(str::to_string).collect(),
        remedy,
        resolved: false,
        bool_description: None,
        syscall_name,
        errno_name,
    })
}

/// 対象リソースの絶対パスを解決する
///
/// 解決の優先順位:
///   1. AVC 行の path= / dest= / name= フィールド（引用符・16進対応）
///   2. 同ブロックの type=PATH レコードの name= フィールド（全レコードを走査）
///   3. デバイス名ヒューリスティック（ptmx, pts 番号など）
///   4. フォールバック: AVC 行から得た raw 値
fn resolve_target(
    avc_line: &str,
    block: &str,
    tclass: &str,
    tcontext: &str,
    path_map: &std::collections::HashMap<String, String>,
) -> String {
    // ステップ 1: AVC 行から候補を取得（引用符・16進対応）
    let raw = extract_field(avc_line, "path=")
        .or_else(|| extract_field(avc_line, "dest="))
        .or_else(|| extract_field(avc_line, "name="))
        .unwrap_or_else(|| tcontext.to_string());

    if raw.starts_with('/') {
        return raw;
    }

    // ステップ 2: 同ブロックの type=PATH レコードを全件走査して絶対パスを探す
    //   複数の PATH レコードがある場合（exec + interpreter など）は
    //   絶対パスを持つ最初のものを採用する
    for line in block.lines().filter(|l| l.contains("type=PATH")) {
        if let Some(p) = extract_field(line, "name=") {
            if p.starts_with('/') {
                return p;
            }
        }
    }

    // ステップ 3: デバイス名ヒューリスティック（ptmx, pts 番号など）
    if let Some(p) = device_path_heuristic(&raw, tclass) {
        return p;
    }

    // ステップ 4: 他ブロックで判明した「ファイル名 → 絶対パス」対応表を参照
    //   execute 拒否など PATH レコードが無いブロックでも、同名ファイルが
    //   他ブロックで解決済みであれば絶対パスを補完できる
    if let Some(abs) = path_map.get(&raw) {
        return abs.clone();
    }

    // ステップ 5: フォールバック
    raw
}

/// デバイス名から絶対パスを推定するヒューリスティック
///
/// audit ログでは chr_file / blk_file の name= が相対名や番号になることがある:
///   - `ptmx`      → /dev/ptmx
///   - `0`, `1`... → /dev/pts/N (chr_file の場合)
///   - その他既知デバイス
fn device_path_heuristic(name: &str, tclass: &str) -> Option<String> {
    // 既知のキャラクタ/ブロックデバイス名
    let known = match name {
        "ptmx"    => "/dev/ptmx",
        "tty"     => "/dev/tty",
        "null"    => "/dev/null",
        "zero"    => "/dev/zero",
        "full"    => "/dev/full",
        "random"  => "/dev/random",
        "urandom" => "/dev/urandom",
        "console" => "/dev/console",
        "kmsg"    => "/dev/kmsg",
        "mem"     => "/dev/mem",
        "port"    => "/dev/port",
        "stdin"   => "/dev/stdin",
        "stdout"  => "/dev/stdout",
        "stderr"  => "/dev/stderr",
        _         => "",
    };
    if !known.is_empty() {
        return Some(known.to_string());
    }

    // chr_file で数字のみ → /dev/pts/N
    if tclass == "chr_file" && !name.is_empty() && name.bytes().all(|b| b.is_ascii_digit()) {
        return Some(format!("/dev/pts/{}", name));
    }

    None
}

/// フィールド値を抽出するヘルパー (key=value 形式)
///
/// audit ログのフィールド値は次の 3 形式がある:
///   1. 引用符付き: `field="value with spaces"` → スペースを含む文字列
///   2. 16進エンコード: `field=2F7573722F...` → 特殊文字を含む文字列
///   3. 通常値: `field=value` → キーワード・数値など
fn extract_field(line: &str, key: &str) -> Option<String> {
    let start = line.find(key)? + key.len();
    let rest = &line[start..];

    Some(if rest.starts_with('"') {
        // 引用符付き: 閉じ引用符まで（スペースを含む値に対応）
        let inner = &rest[1..];
        let end = inner.find('"').unwrap_or(inner.len());
        inner[..end].to_string()
    } else {
        // 空白または行末まで取得
        let end = rest.find(|c: char| c == ' ' || c == '\n').unwrap_or(rest.len());
        let raw = &rest[..end];
        // 16進エンコードの場合はデコード、そうでなければそのまま返す
        try_decode_hex(raw).unwrap_or_else(|| raw.to_string())
    })
}

/// audit ログの 16 進エンコードされたフィールド値をデコードする
///
/// 条件: 偶数長 4 文字以上・全文字が ASCII 16 進数字・デコード後が有効な UTF-8
/// かつ制御文字（タブ・改行を除く）を含まない場合のみデコードを返す
fn try_decode_hex(s: &str) -> Option<String> {
    if s.len() < 4 || s.len() % 2 != 0 {
        return None;
    }
    if !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let bytes: Vec<u8> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("already validated hex"))
        .collect();
    let decoded = String::from_utf8(bytes).ok()?;
    // 制御文字を含む場合は 16 進エンコードではないと判定
    if decoded.bytes().any(|b| b < 0x20 && b != b'\t' && b != b'\n') {
        return None;
    }
    Some(decoded)
}

/// AVC 情報から解決策候補を判定する
pub fn diagnose_remedy(
    perm: &str,
    tclass: &str,
    scontext: &str,
    tcontext: &str,
    _target: &str,
) -> Remedy {
    let ttype = tcontext.split(':').nth(2).unwrap_or("");

    // ポートコンテキスト
    if matches!(tclass, "tcp_socket" | "udp_socket" | "rawip_socket")
        && matches!(perm, "name_bind" | "name_connect")
    {
        return Remedy::PortContext;
    }

    // ファイル・ディレクトリ系
    if matches!(tclass, "file" | "dir" | "lnk_file") {
        // ラベルなし → restorecon
        if ttype == "unlabeled_t" {
            return Remedy::Restorecon;
        }

        // execute_no_trans: ドメイン遷移ルールが未定義
        if perm.contains("execute_no_trans") {
            return Remedy::CustomPolicy;
        }

        // ttype が「汎用型・仮ラベル」ならファイルコンテキストの設定が必要
        // それ以外の具体的なポリシー型ならラベルは正しくポリシー不足
        return if is_generic_type(ttype) {
            Remedy::FileContext
        } else {
            Remedy::CustomPolicy
        };
    }

    // Boolean マップ（代表的なものだけ列挙）
    let boolean_map: &[(&str, &str, &str)] = &[
        ("httpd_t", "name_connect", "httpd_can_network_connect"),
        ("httpd_t", "write",        "httpd_anon_write"),
        ("ftpd_t",  "read",         "ftp_home_dir"),
        ("samba_t", "read",         "samba_enable_home_dirs"),
    ];
    for (sctx_frag, p, bool_name) in boolean_map {
        if scontext.contains(sctx_frag) && perm.contains(p) {
            return Remedy::Boolean(bool_name.to_string());
        }
    }

    Remedy::CustomPolicy
}

/// x86_64 システムコール番号をシンボル名に変換する
fn syscall_name_from_nr(nr: u32) -> Option<&'static str> {
    Some(match nr {
        0   => "read",          1   => "write",         2   => "open",
        3   => "close",         4   => "stat",          5   => "fstat",
        6   => "lstat",         7   => "poll",          8   => "lseek",
        9   => "mmap",          10  => "mprotect",      11  => "munmap",
        12  => "brk",           16  => "ioctl",         17  => "pread64",
        18  => "pwrite64",      19  => "readv",         20  => "writev",
        21  => "access",        22  => "pipe",          23  => "select",
        32  => "dup",           33  => "dup2",          39  => "getpid",
        40  => "sendfile",      41  => "socket",        42  => "connect",
        43  => "accept",        44  => "sendto",        45  => "recvfrom",
        46  => "sendmsg",       47  => "recvmsg",       48  => "shutdown",
        49  => "bind",          50  => "listen",        51  => "getsockname",
        52  => "getpeername",   53  => "socketpair",    54  => "setsockopt",
        55  => "getsockopt",    56  => "clone",         57  => "fork",
        58  => "vfork",         59  => "execve",        60  => "exit",
        61  => "wait4",         62  => "kill",          63  => "uname",
        72  => "fcntl",         74  => "fsync",         75  => "fdatasync",
        76  => "truncate",      77  => "ftruncate",     78  => "getdents",
        79  => "getcwd",        80  => "chdir",         81  => "fchdir",
        82  => "rename",        83  => "mkdir",         84  => "rmdir",
        85  => "creat",         86  => "link",          87  => "unlink",
        88  => "symlink",       89  => "readlink",      90  => "chmod",
        91  => "fchmod",        92  => "chown",         93  => "fchown",
        94  => "lchown",        95  => "umask",         97  => "getrlimit",
        101 => "ptrace",        102 => "getuid",        105 => "setuid",
        106 => "setgid",        107 => "geteuid",       108 => "getegid",
        133 => "mknod",         157 => "prctl",         186 => "gettid",
        217 => "getdents64",    232 => "exit_group",    233 => "epoll_wait",
        234 => "epoll_ctl",     257 => "openat",        258 => "mkdirat",
        259 => "mknodat",       260 => "fchownat",      261 => "futimesat",
        262 => "newfstatat",    263 => "unlinkat",      264 => "renameat",
        265 => "linkat",        266 => "symlinkat",     267 => "readlinkat",
        268 => "fchmodat",      269 => "faccessat",     280 => "utimensat",
        281 => "epoll_pwait",   285 => "fallocate",     288 => "accept4",
        291 => "epoll_create1", 292 => "dup3",          293 => "pipe2",
        316 => "renameat2",     318 => "getrandom",     319 => "memfd_create",
        322 => "execveat",      332 => "statx",         435 => "clone3",
        437 => "openat2",       439 => "faccessat2",
        _   => return None,
    })
}

/// Linux errno 番号をシンボル名に変換する（負の exit 値の絶対値を渡す）
fn errno_name_from_code(code: u32) -> Option<&'static str> {
    Some(match code {
        1   => "EPERM",         2   => "ENOENT",        3   => "ESRCH",
        4   => "EINTR",         5   => "EIO",           6   => "ENXIO",
        7   => "E2BIG",         8   => "ENOEXEC",       9   => "EBADF",
        10  => "ECHILD",        11  => "EAGAIN",        12  => "ENOMEM",
        13  => "EACCES",        14  => "EFAULT",        16  => "EBUSY",
        17  => "EEXIST",        18  => "EXDEV",         19  => "ENODEV",
        20  => "ENOTDIR",       21  => "EISDIR",        22  => "EINVAL",
        23  => "ENFILE",        24  => "EMFILE",        25  => "ENOTTY",
        26  => "ETXTBSY",       28  => "ENOSPC",        29  => "ESPIPE",
        30  => "EROFS",         32  => "EPIPE",         33  => "EDOM",
        34  => "ERANGE",        35  => "EDEADLK",       36  => "ENAMETOOLONG",
        38  => "ENOSYS",        39  => "ENOTEMPTY",     40  => "ELOOP",
        95  => "EOPNOTSUPP",    97  => "EAFNOSUPPORT",  98  => "EADDRINUSE",
        99  => "EADDRNOTAVAIL", 101 => "ENETUNREACH",   104 => "ECONNRESET",
        107 => "ENOTCONN",      110 => "ETIMEDOUT",     111 => "ECONNREFUSED",
        113 => "EHOSTUNREACH",  115 => "EINPROGRESS",
        _   => return None,
    })
}

/// SELinux の型が「汎用型・仮ラベル」かどうかを判定する
///
/// 汎用型の場合はファイルコンテキストの設定（semanage fcontext）が解決策候補。
/// 具体的なポリシー型（*_exec_t や各サービス専用型）の場合はラベルが正しいため
/// ポリシー追加（audit2allow）が必要。
fn is_generic_type(ttype: &str) -> bool {
    // 明示的に汎用とわかっている型
    const GENERIC_TYPES: &[&str] = &[
        "default_t",       // ラベルが未設定のファイルに付くデフォルト型
        "unlabeled_t",     // ラベルなし
        "file_t",          // 未ラベルファイルシステム
        "bin_t",           // 汎用バイナリ（専用 *_exec_t が望ましい）
        "lib_t",           // 汎用ライブラリ
        "usr_t",           // /usr 以下の汎用型
        "etc_t",           // /etc 以下の汎用型
        "var_t",           // /var 以下の汎用型
        "tmp_t",           // /tmp 以下
        "tmpfs_t",         // tmpfs
        "user_tmp_t",      // ユーザー一時ファイル
        "home_root_t",     // /home そのもの
        "user_home_t",     // ユーザーホームディレクトリ
        "staff_home_t",
        "admin_home_t",
        "nfs_t",           // NFS マウント
        "cifs_t",          // Samba/CIFS マウント
        "removable_t",     // リムーバブルメディア
        "dosfs_t",         // FAT ファイルシステム
    ];

    GENERIC_TYPES.contains(&ttype)
}
