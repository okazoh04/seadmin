# seadmin

[日本語](README.md) | English | [中文](README.zh.md)

A Rust-based TUI tool for reviewing and remediating SELinux access denials.

## Overview

Parses `ausearch` output to interactively list access denials, analyze their causes, and execute remediation commands.

- Aggregated list of access denials (process / action / target / count)
- Automatic root-cause analysis with suggested remediation
- Direct execution of `restorecon` / `semanage fcontext` / `setsebool` / `audit2allow`
- Session-scoped sudo password cache to minimize re-entry
- Operation log saved to `~/.local/share/seadmin/seadmin.log`
- **Multilingual** (Japanese / English / Chinese) — auto-detected from `LANG`

## Screenshot

```
┌─ Access Denials  [Today]  Unresolved: 3 / Total: 3 ───────────────────────────┐
│ #  When       Process       Action        Target                 Count Remedy   │
│▶1  3m ago     nginx         name_bind     /var/run/nginx.sock      2  Port Context│
│ 2  1h ago     httpd         write         /var/www/html/upload     5  File Context│
│ 3  2d ago     mysqld_t      name_connect  192.168.1.100:3306       1  Boolean:...│
└────────────────────────────────────────────────────────────────────────────────┘
[seadmin] [Enforcing] hostname  ↑↓/jk:Move  Enter:Detail  /:Filter  r:Reload  q:Quit
```

## Requirements

- Linux with SELinux enabled (Fedora / RHEL / CentOS / Rocky Linux, etc.)
- Rust 1.85 or later (edition 2024)
- The following commands must be in `PATH`:
  - `ausearch` (audit package)
  - `getenforce` (libselinux-utils)
  - `sudo`
  - `semanage` (policycoreutils-python-utils) — for remediation
  - `audit2allow` (policycoreutils-devel) — for custom policy generation
  - `restorecon` (policycoreutils) — for label repair
  - `setsebool` (libselinux-utils) — for Boolean changes
  - `semodule` (policycoreutils) — for policy module installation

## Installation

```bash
git clone https://github.com/yourusername/seadmin.git
cd seadmin

# Release build (strip + LTO + size optimized)
cargo build --release

# Install binary
sudo install -m 755 target/release/seadmin /usr/local/bin/
```

## Usage

```bash
seadmin
```

If `ausearch` requires root privileges on your system:

```bash
sudo seadmin
```

### Language selection

The display language is determined automatically from the `LANG` environment variable.

| LANG value | Language |
|------------|----------|
| `ja_*` | Japanese (default) |
| `zh_*` | Chinese (Simplified) |
| other | English |

## Key Bindings

### Access Denial List

| Key | Action |
|-----|--------|
| `↑` / `k` | Move cursor up |
| `↓` / `j` | Move cursor down |
| `Enter` | Open detail / remediation screen |
| `/` | Start filter input (process / action / target) |
| `Esc` | Clear filter |
| `r` | Reload access denial log |
| `l` | Toggle operation log overlay |
| `q` | Quit |
| `Ctrl+C` | Force quit |

### Detail / Remediation Screen

| Key | Action |
|-----|--------|
| `↑` / `k` | Move option selection up |
| `↓` / `j` | Move option selection down |
| `A`–`F` | Select remediation option directly |
| `Enter` | Execute selected remediation (sudo auth popup) |
| `Esc` / `←` | Return to list |

### Policy Review Screen (audit2allow)

| Key | Action |
|-----|--------|
| `↑` / `k` | Scroll up |
| `↓` / `j` | Scroll down |
| `Enter` | Apply policy to the system |
| `Esc` | Cancel (deletes generated file) |

## Remediation Options

| Option | Description |
|--------|-------------|
| **Port Context** | Add port context with `semanage port -a` |
| **File Context** | Add file context rule with `semanage fcontext -a` |
| **restorecon** | Restore default context with `restorecon -Rv` |
| **Boolean** | Enable SELinux Boolean with `setsebool` (temporary / persistent) |
| **Custom Policy** | Auto-generate, review, and apply policy module with `audit2allow` |
| **Permissive** | Set domain to Permissive with `semanage permissive -a` (investigation only) |

## Security

- sudo passwords are managed with the [`zeroize`](https://docs.rs/zeroize) crate and zeroed in memory on drop
- 3 consecutive authentication failures trigger a 60-second lockout
- Passwords are passed via stdin to `sudo -S -k` and never appear in process arguments

## Development

```bash
# Development build
cargo build

# Tests
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## License

GPL-3.0
