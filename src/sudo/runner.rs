/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

use anyhow::{bail, Result};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use zeroize::Zeroizing;

/// sudo でコマンドを実行する
/// `password` は実行直後に zeroize される
pub async fn run_with_sudo(
    args: &[&str],
    password: Zeroizing<String>,
) -> Result<SudoResult> {
    if args.is_empty() {
        bail!("コマンドが空です");
    }

    // sudo -S -- <args>  (-k を外してセッション内キャッシュを活用)
    let mut child = Command::new("sudo")
        .args(["-S", "--"])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // パスワードを stdin に書き込む
    if let Some(mut stdin) = child.stdin.take() {
        let pw_line = format!("{}\n", *password);
        stdin.write_all(pw_line.as_bytes()).await?;
        // パスワードは drop 時に zeroize される（Zeroizing<String> の保証）
    }
    // password はここで drop → zeroize

    let output = child.wait_with_output().await?;

    if output.status.success() {
        Ok(SudoResult::Ok)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        // 認証失敗の判定
        if stderr.contains("incorrect password")
            || stderr.contains("authentication failure")
            || stderr.contains("Sorry, try again")
        {
            Ok(SudoResult::AuthFailed)
        } else {
            Ok(SudoResult::CommandFailed { stderr })
        }
    }
}

#[derive(Debug)]
pub enum SudoResult {
    Ok,
    AuthFailed,
    CommandFailed { stderr: String },
}
