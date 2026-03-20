/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use anyhow::Result;
use std::io::Write;
use std::process::Stdio;
use tokio::process::Command;

/// ausearch を実行して生の出力を返す
/// audit.log が読めない場合は sudo にフォールバック
pub async fn ausearch_avc() -> Result<String> {
    // ausearch は結果0件でも exit code 1 を返すため、
    // stderr に "permission denied" 系のメッセージがあるときだけ権限エラーと判定する
    let args = ["-m", "AVC,USER_AVC,SELINUX_ERR", "-ts", "today"];

    let out = Command::new("ausearch")
        .env("LC_ALL", "C")
        .args(args)
        .output()
        .await;

    match out {
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            // 権限エラー以外は stdout を返す（<no matches> も正常応答）
            // LC_ALL=C なので英語のみチェック
            if !stderr.to_lowercase().contains("permission denied") {
                return Ok(String::from_utf8_lossy(&o.stdout).to_string());
            }
        }
        Err(_) => {}
    }

    // sudo にフォールバック（パスワード不要の場合のみ）
    let out = Command::new("sudo")
        .env("LC_ALL", "C")
        .args(["-n", "ausearch"])
        .args(args)
        .output()
        .await?;

    let stderr = String::from_utf8_lossy(&out.stderr);
    // sudo が "password is required" を含む場合は権限不足と判定
    if stderr.contains("password is required") {
        let lang = crate::i18n::detect_lang();
        Err(anyhow::anyhow!("{}", lang.err_audit_no_perm()))
    } else {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

/// getenforce の結果を返す
pub async fn getenforce() -> Result<String> {
    let out = Command::new("getenforce")
        .env("LC_ALL", "C")
        .output()
        .await?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// ホスト名を返す
pub async fn hostname() -> Result<String> {
    let out = Command::new("hostname")
        .env("LC_ALL", "C")
        .output()
        .await?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}


/// `ausearch` の raw 出力（AVC 行）を `audit2allow -w` に渡して原因カテゴリを返す
///
/// 戻り値:
///   - `Some(("BOOLEAN", "bool_name"))` — boolean が off
///   - `Some(("BADTCON", ""))` — ターゲットのラベル不正
///   - `Some(("TERULE", ""))` — TE allow ルール不足
///   - `Some(("CONSTRAINT", ""))` — MLS/MCS 制約違反
///   - `None` — 判定不能 / コマンドなし
pub async fn audit2why(avc_lines: &[String]) -> Option<(String, String)> {
    use tokio::io::AsyncWriteExt;

    let mut child = tokio::process::Command::new("audit2allow")
        .env("LC_ALL", "C")
        .args(["-w"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = child.stdin.take() {
        let input = avc_lines.join("\n");
        let _ = stdin.write_all(input.as_bytes()).await;
    }

    let output = child.wait_with_output().await.ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    parse_audit2why_output(&text)
}

/// `audit2allow -w` の出力を解析して原因カテゴリを返す
///
/// setroubleshoot の解析ロジックに準拠:
///   "Was caused by:" が見つかるまでスキャンし、その後の
///   "Missing type enforcement (TE) allow rule" → TERULE
///   "The boolean"                               → BOOLEAN + bool 名
///   "Incorrect Target Context Label"            → BADTCON  (英語)
///   "target context label"                      → BADTCON  (別表現)
///   "MLS"                                       → CONSTRAINT
fn parse_audit2why_output(text: &str) -> Option<(String, String)> {
    // boolean 名の抽出（優先度高）
    for line in text.lines() {
        if let Some(bool_name) = extract_boolean_name(line) {
            return Some(("BOOLEAN".to_string(), bool_name));
        }
    }

    // その他の理由（複数の行にまたがる可能性があるため、行単位でキーワードスキャン）
    for line in text.lines() {
        let l = line.to_lowercase();
        // TERULE: ルール不足
        if l.contains("missing type enforcement") || (l.contains("allow") && l.contains("rule")) {
            return Some(("TERULE".to_string(), String::new()));
        }
        // BADTCON: ラベル不正
        if l.contains("incorrect target context label") || l.contains("target context label") {
            return Some(("BADTCON".to_string(), String::new()));
        }
        // CONSTRAINT: 制約
        if l.contains("mls") || l.contains("mcs") || l.contains("constraint") {
            return Some(("CONSTRAINT".to_string(), String::new()));
        }
    }
    None
}

/// audit2why 出力から boolean 名を抽出する
/// "The boolean X was set incorrectly" / "turn on the X boolean"
fn extract_boolean_name(line: &str) -> Option<String> {
    // パターン 1: "The boolean <name> was set incorrectly"
    if let Some(rest) = line.strip_prefix("The boolean ") {
        let name = rest.split_whitespace().next()?;
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    // パターン 2: "you must turn on the <name> boolean"
    if line.contains("turn on the ") && line.contains(" boolean") {
        let start = line.find("turn on the ")? + "turn on the ".len();
        let rest = &line[start..];
        let name = rest.split_whitespace().next()?;
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    None
}

/// `sepolicy booleans -b BOOL_NAME` を実行して説明文を返す
/// sepolicy が存在しない場合は None を返す
pub async fn sepolicy_bool_desc(bool_name: &str) -> Option<String> {
    let out = Command::new("sepolicy")
        .env("LC_ALL", "C")
        .args(["booleans", "-b", bool_name])
        .output()
        .await
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    // 出力形式: "BOOL_NAME - Description text"
    for line in text.lines() {
        if let Some(desc) = line.splitn(2, " - ").nth(1) {
            let d = desc.trim().to_string();
            if !d.is_empty() {
                return Some(d);
            }
        }
    }
    None
}

/// audit2allow -M でポリシーを生成し、(.te の内容, .pp ファイルパス) を返す
/// .pp ファイルは /tmp にランダムな名前で作成して返す。呼び出し元が semodule -i 後に削除すること。
pub async fn audit2allow_generate(avc_lines: &[String], module_name: &str) -> Result<(String, String)> {
    use tokio::io::AsyncWriteExt;
    use tempfile::tempdir;

    // 予測不可能な一時ディレクトリを作成
    let tmp_dir = tempdir()?;
    let tmp_path = tmp_dir.path();

    let mut child = Command::new("audit2allow")
        .env("LC_ALL", "C")
        .args(["-M", module_name])
        .current_dir(tmp_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        let input = avc_lines.join("\n");
        stdin.write_all(input.as_bytes()).await?;
    }
    let output = child.wait_with_output().await?;
    if !output.status.success() {
        let lang = crate::i18n::detect_lang();
        return Err(anyhow::anyhow!(
            "{}",
            lang.err_audit2allow_failed(&String::from_utf8_lossy(&output.stderr))
        ));
    }

    let te_path = tmp_path.join(format!("{}.te", module_name));
    let pp_path_src = tmp_path.join(format!("{}.pp", module_name));

    let te = tokio::fs::read_to_string(&te_path).await.unwrap_or_else(|_| {
        String::from_utf8_lossy(&output.stdout).to_string()
    });

    // .pp ファイルを安全な一時ファイル名に移動する
    // /tmp 直下にランダムな名前で作成し、永続化（keep）させる
    let (mut temp_file, dest_pp_path) = tempfile::Builder::new()
        .prefix("seadmin-")
        .suffix(".pp")
        .tempfile_in("/tmp")?
        .keep()?;

    let pp_content = tokio::fs::read(&pp_path_src).await?;
    temp_file.write_all(&pp_content)?;
    temp_file.flush()?;

    Ok((te, dest_pp_path.to_string_lossy().to_string()))
}

/// ポリシーモジュール情報
#[derive(Debug, Clone)]
pub struct PolicyModule {
    pub name: String,
    pub priority: u16,
}

/// `semodule -lfull` を実行してモジュール一覧を返す
/// 出力形式: "priority  name  type" (例: "400  base  pp")
pub async fn semodule_list_full() -> Result<Vec<PolicyModule>> {
    let out = Command::new("semodule")
        .env("LC_ALL", "C")
        .args(["-lfull"])
        .output()
        .await?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut modules = Vec::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(priority) = parts[0].parse::<u16>() {
                modules.push(PolicyModule {
                    name: parts[1].to_string(),
                    priority,
                });
            }
        }
    }
    modules.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.name.cmp(&b.name)));
    Ok(modules)
}
