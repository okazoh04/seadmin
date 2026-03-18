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
use std::process::Stdio;
use tokio::process::Command;

/// ausearch を実行して生の出力を返す
/// audit.log が読めない場合は sudo にフォールバック
pub async fn ausearch_avc() -> Result<String> {
    // ausearch は結果0件でも exit code 1 を返すため、
    // stderr に "permission denied" 系のメッセージがあるときだけ権限エラーと判定する
    let args = ["-m", "AVC,USER_AVC,SELINUX_ERR", "-ts", "today"];

    let out = Command::new("ausearch").args(args).output().await;

    match out {
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            // 権限エラー以外は stdout を返す（<no matches> も正常応答）
            // OS が出力する "Permission denied" / "許可がありません" の両方を検出する
            if !stderr.contains("許可がありません") && !stderr.contains("Permission denied") {
                return Ok(String::from_utf8_lossy(&o.stdout).to_string());
            }
        }
        Err(_) => {}
    }

    // sudo にフォールバック（パスワード不要の場合のみ）
    let out = Command::new("sudo")
        .args(["-n", "ausearch"])
        .args(args)
        .output()
        .await?;

    let stderr = String::from_utf8_lossy(&out.stderr);
    // sudo が "password is required" / "パスワード" を含む場合は権限不足と判定
    if stderr.contains("password is required") || stderr.contains("パスワード") {
        let lang = crate::i18n::detect_lang();
        Err(anyhow::anyhow!("{}", lang.err_audit_no_perm()))
    } else {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

/// getenforce の結果を返す
pub async fn getenforce() -> Result<String> {
    let out = Command::new("getenforce").output().await?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// ホスト名を返す
pub async fn hostname() -> Result<String> {
    let out = Command::new("hostname").output().await?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}


/// audit2allow -M でポリシーを生成し、(.te の内容, .pp ファイルパス) を返す
/// .pp ファイルは /tmp に置いたまま返す。呼び出し元が semodule -i 後に削除すること。
pub async fn audit2allow_generate(avc_lines: &[String], module_name: &str) -> Result<(String, String)> {
    use tokio::io::AsyncWriteExt;

    let tmp = std::path::Path::new("/tmp");
    let mut child = Command::new("audit2allow")
        .args(["-M", module_name])
        .current_dir(tmp)
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

    let te_path = tmp.join(format!("{}.te", module_name));
    let pp_path = tmp.join(format!("{}.pp", module_name));

    let te = tokio::fs::read_to_string(&te_path).await.unwrap_or_else(|_| {
        String::from_utf8_lossy(&output.stdout).to_string()
    });
    let _ = tokio::fs::remove_file(&te_path).await;
    // .pp は semodule -i 後に呼び出し元が削除する

    Ok((te, pp_path.to_string_lossy().to_string()))
}
