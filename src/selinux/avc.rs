use chrono::{DateTime, Local};
use std::fmt;

/// AVC デナイアルの解決策候補
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
            Remedy::PortContext => write!(f, "ポート追加"),
            Remedy::FileContext => write!(f, "fcontext変更"),
            Remedy::Restorecon => write!(f, "restorecon"),
            Remedy::Boolean(b) => write!(f, "Boolean: {}", b),
            Remedy::CustomPolicy => write!(f, "カスタムポリシー"),
        }
    }
}

/// 1件の AVC デナイアル（集計済み）
#[derive(Debug, Clone)]
pub struct AvcEntry {
    pub id: usize,
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
}

impl AvcEntry {
    /// 相対時刻の文字列表現（例: "3時間前"）
    pub fn elapsed_str(&self) -> String {
        let now = Local::now();
        let secs = (now - self.last_seen).num_seconds().max(0) as u64;
        if secs < 60 {
            format!("{}秒前", secs)
        } else if secs < 3600 {
            format!("{}分前", secs / 60)
        } else if secs < 86400 {
            format!("{}時間前", secs / 3600)
        } else {
            format!("{}日前", secs / 86400)
        }
    }
}

/// `ausearch -m AVC,USER_AVC,SELINUX_ERR -i` の出力を解析して AVC エントリ一覧を返す
pub fn parse_ausearch_output(raw: &str) -> Vec<AvcEntry> {
    // ausearch --interpret 出力は "----" でレコードが区切られる
    let blocks: Vec<&str> = raw.split("----").collect();
    let mut map: std::collections::HashMap<String, AvcEntry> = std::collections::HashMap::new();
    let mut next_id = 1usize;

    for block in &blocks {
        if let Some(entry) = parse_block(block, next_id) {
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

fn parse_block(block: &str, id: usize) -> Option<AvcEntry> {
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

    // 対象リソース: path= は絶対パス、name= はファイル名のみなので path= を優先
    let target_raw = extract_field(avc_line, "path=")
        .or_else(|| extract_field(avc_line, "dest="))
        .or_else(|| extract_field(avc_line, "name="))
        .unwrap_or_else(|| tcontext.clone());
    let target_raw = target_raw.trim_matches('"');

    // AVC 行が絶対パスでない場合、同ブロックの PATH レコードから絶対パスを補完する
    let target = if target_raw.starts_with('/') {
        target_raw.to_string()
    } else {
        block
            .lines()
            .filter(|l| l.contains("type=PATH"))
            .find_map(|l| {
                let p = extract_field(l, "name=")?;
                let p = p.trim_matches('"').to_string();
                if p.starts_with('/') { Some(p) } else { None }
            })
            .unwrap_or_else(|| target_raw.to_string())
    };

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

    Some(AvcEntry {
        id,
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
    })
}

/// フィールド値を抽出するヘルパー (key=value 形式、空白またはEOLまで)
fn extract_field<'a>(line: &'a str, key: &str) -> Option<String> {
    let start = line.find(key)? + key.len();
    let rest = &line[start..];
    let end = rest.find(|c: char| c == ' ' || c == '\n').unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

/// AVC 情報から解決策候補を判定する
pub fn diagnose_remedy(
    perm: &str,
    tclass: &str,
    _scontext: &str,
    _tcontext: &str,
    _target: &str,
) -> Remedy {
    // ポートコンテキスト
    if (tclass == "tcp_socket" || tclass == "udp_socket")
        && (perm == "name_bind" || perm == "name_connect")
    {
        return Remedy::PortContext;
    }

    // ファイルコンテキスト系
    if matches!(tclass, "file" | "dir" | "lnk_file") {
        // restorecon で直る可能性があるか（簡易判定）
        if _tcontext.contains("unlabeled_t") {
            return Remedy::Restorecon;
        }
        return Remedy::FileContext;
    }

    // Boolean マップ（代表的なものだけ列挙）
    let boolean_map: &[(&str, &str, &str)] = &[
        ("httpd_t", "name_connect", "httpd_can_network_connect"),
        ("httpd_t", "write", "httpd_anon_write"),
        ("ftpd_t", "read", "ftp_home_dir"),
        ("samba_t", "read", "samba_enable_home_dirs"),
    ];
    for (sctx_frag, p, bool_name) in boolean_map {
        if _scontext.contains(sctx_frag) && perm.contains(p) {
            return Remedy::Boolean(bool_name.to_string());
        }
    }

    Remedy::CustomPolicy
}
