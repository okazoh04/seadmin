# seadmin

[日本語](README.md) | [English](README.en.md) | 中文

基于 Rust 的 TUI 工具，用于查看和处理 SELinux 访问拒绝事件。

## 概述

解析 `ausearch` 输出，以交互方式列出访问拒绝事件、分析原因并执行修复命令。

- 访问拒绝事件的汇总列表（进程 / 操作 / 目标 / 次数）
- 自动根因分析并提供修复建议
- 直接执行 `restorecon` / `semanage fcontext` / `setsebool` / `audit2allow`
- 会话内缓存 sudo 密码，减少重复输入
- 操作日志保存至 `~/.local/share/seadmin/seadmin.log`
- **多语言支持**（日语 / 英语 / 中文）— 根据 `LANG` 自动切换

## 截图

```
┌─ 访问拒绝列表  [今日]  未处理: 3条 / 共 3条 ──────────────────────────────────┐
│ #  时间       进程          操作          目标                   次数 修复方案   │
│▶1  3分前      nginx         name_bind     /var/run/nginx.sock      2  添加端口   │
│ 2  1小时前    httpd         write         /var/www/html/upload     5  fcontext更改│
│ 3  2天前      mysqld_t      name_connect  192.168.1.100:3306       1  Boolean:...│
└────────────────────────────────────────────────────────────────────────────────┘
[seadmin] [Enforcing] hostname  ↑↓/jk:移动  Enter:详情  /:过滤  r:刷新  q:退出
```

## 环境要求

- 已启用 SELinux 的 Linux 发行版（Fedora / RHEL / CentOS / Rocky Linux 等）
- Rust 1.85 及以上（edition 2024）
- 以下命令需在 `PATH` 中：
  - `ausearch`（audit 软件包）
  - `getenforce`（libselinux-utils）
  - `sudo`
  - `semanage`（policycoreutils-python-utils）— 用于修复操作
  - `audit2allow`（policycoreutils-devel）— 用于生成自定义策略
  - `restorecon`（policycoreutils）— 用于修复标签
  - `setsebool`（libselinux-utils）— 用于修改 Boolean
  - `semodule`（policycoreutils）— 用于安装策略模块

## 安装

```bash
git clone https://github.com/yourusername/seadmin.git
cd seadmin

# 发布构建（strip + LTO + 体积优化）
cargo build --release

# 安装二进制文件
sudo install -m 755 target/release/seadmin /usr/local/bin/
```

## 使用方法

```bash
seadmin
```

若 `ausearch` 需要 root 权限：

```bash
sudo seadmin
```

### 语言切换

显示语言根据 `LANG` 环境变量自动决定。

| LANG 值 | 语言 |
|---------|------|
| `ja_*` | 日语 |
| `zh_TW_*` / `zh_HK_*` / `zh_MO_*` | 中文（繁体） |
| `zh_*` | 中文（简体） |
| `ko_*` | 韩语 |
| `ru_*` | 俄语 |
| `kk_*` | 哈萨克语 |
| `es_*` | 西班牙语 |
| `pt_*` | 葡萄牙语 |
| `fr_*` | 法语 |
| `de_*` | 德语 |
| `it_*` | 意大利语 |
| `nl_*` | 荷兰语 |
| `sv_*` | 瑞典语 |
| `nb_*` / `nn_*` / `no_*` | 挪威语 |
| `ar_*` | 阿拉伯语 |
| `th_*` | 泰语 |
| `vi_*` | 越南语 |
| 其他 | 英语（默认） |

## 快捷键

### 访问拒绝列表画面

| 按键 | 操作 |
|------|------|
| `↑` / `k` | 光标上移 |
| `↓` / `j` | 光标下移 |
| `Enter` | 打开详情/处置画面 |
| `/` | 开始过滤输入（进程 / 操作 / 目标） |
| `Esc` | 清除过滤 |
| `r` | 重新加载访问拒绝日志 |
| `l` | 显示操作日志覆盖层（按 Esc 关闭） |
| `q` | 退出 |
| `Ctrl+C` | 强制退出 |

### 详情/处置画面

| 按键 | 操作 |
|------|------|
| `↑` / `k` | 选项向上移动 |
| `↓` / `j` | 选项向下移动 |
| `A`～`F` | 直接按键选择处置选项 |
| `Enter` | 执行选定处置（sudo 认证弹窗） |
| `Esc` / `←` | 返回列表 |

### 策略审核画面（audit2allow）

| 按键 | 操作 |
|------|------|
| `↑` / `k` | 向上滚动 |
| `↓` / `j` | 向下滚动 |
| `Enter` | 将策略应用到系统 |
| `Esc` | 取消（删除生成的文件） |

## 修复选项

| 选项 | 说明 |
|------|------|
| **添加端口** | 使用 `semanage port -a` 添加端口上下文 |
| **fcontext 更改** | 使用 `semanage fcontext -a` 添加文件上下文规则 |
| **restorecon** | 使用 `restorecon -Rv` 恢复默认上下文 |
| **Boolean** | 使用 `setsebool` 启用 SELinux Boolean（临时 / 持久） |
| **自定义策略** | 使用 `audit2allow` 自动生成、审核并应用策略模块 |
| **Permissive 设置** | 使用 `semanage permissive -a` 临时将域设为 Permissive（仅用于调查） |

## 安全性与健壮性

- **内存保护:** sudo 密码由 [`zeroize`](https://docs.rs/zeroize) crate 管理，在释放（drop）时内存清零。
- **暴力破解防护:** 连续认证失败 3 次将触发 60 秒锁定。
- **安全执行:** 密码通过 stdin 传递给 `sudo -S -k`，不出现在进程参数中。
- **区域隔离:** 在执行外部命令时强制使用 `LC_ALL=C`，以实现不受系统语言设置影响的稳定解析。
- **安全临时文件:** `audit2allow` 的中间产物使用 `tempfile` 生成，通过不可预测的路径防止冲突或攻击。
- **遵循 XDG 规范:** 日志和数据保存路径遵循 XDG Base Directory 规范（使用 `dirs` crate）。

## 开发

```bash
# 开发构建
cargo build

# 测试
cargo test

# Lint
cargo clippy

# 格式化
cargo fmt
```

## 许可证

Copyright (C) 2026 okazoh04

本项目基于 [GNU General Public License v3.0](LICENSE) 发布。

```
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
```
