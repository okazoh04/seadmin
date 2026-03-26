/*
 * SPDX-License-Identifier: GPL-3.0-only
 *
 * Copyright (c) 2026 okazoh04
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3.
 * See the LICENSE file for details.
 */

// ── 하단 힌트 ─────────────────────────────────────────────────────────────────
pub const HINT_AVC_LIST:     &str = "↑↓/jk:이동  Enter:상세  /:필터  r:새로고침  m:모듈  l:로그  q:종료";
pub const HINT_AVC_DETAIL:   &str = "A-F:선택  Esc/←:뒤로  Enter:확인";
pub const HINT_POLICY_REVIEW:&str = "↑↓/jk:스크롤  Enter:적용  Esc:취소";
pub const HINT_AUTH:         &str = "Enter:실행  Esc:취소";
pub const HINT_MODULE_LIST:  &str = "↑↓/jk:이동  d:삭제  Esc:뒤로";

// ── 테이블 헤더 ───────────────────────────────────────────────────────────────
pub const COL_OCCURRED: &str = "발생시각";
pub const COL_PROCESS:  &str = "프로세스";
pub const COL_ACTION:   &str = "작업";
pub const COL_TARGET:   &str = "대상";
pub const COL_COUNT:    &str = "횟수";
pub const COL_REMEDY:       &str = "해결방법";
pub const COL_PRIORITY:     &str = "우선순위";
pub const COL_MODULE_NAME:  &str = "모듈명";

// ── 상태 / 메시지 ─────────────────────────────────────────────────────────────
pub const LOADING_MSG:      &str = " ⏳ AVC 로그 로딩 중...";
pub const SELINUX_DISABLED: &str = "⚠ SELinux가 비활성화되어 있습니다. 접근 거부가 기록되지 않습니다.";
pub const NO_AVC:           &str = "접근 거부 없음";
pub const OP_COMPLETE:      &str = "작업 완료";
pub const IGNORED:          &str = "무시 목록에 추가됨";
pub const FILTER_LABEL:     &str = "/필터: ";

// ── 인증 팝업 ─────────────────────────────────────────────────────────────────
pub const AUTH_TITLE:      &str = "🔒 관리자 인증";
pub const AUTH_CMD_LABEL:  &str = "  명령어:";
pub const AUTH_PW_LABEL:   &str = "  비밀번호:";
pub const AUTH_CANCEL_BTN: &str = "[ 취소 (Esc) ]";
pub const AUTH_EXEC_BTN:   &str = "[ 실행 (Enter) ]";
pub const PW_WRONG:        &str = "비밀번호가 올바르지 않습니다";

// ── 상세 화면 블록 ────────────────────────────────────────────────────────────
pub const BLOCK_ANALYSIS: &str = " 분석 ";
pub const BLOCK_OPTIONS:  &str = " 해결 옵션 ";
pub const BLOCK_RAW_LOG:  &str = " 원시 로그 (참조용)";

// ── 정책 검토 ─────────────────────────────────────────────────────────────────
pub const POLICY_REVIEW_TITLE: &str = " 정책 검토 (Enter:적용  Esc:취소)";
pub const POLICY_APPLY_DESC:   &str = "생성된 정책 모듈을 시스템에 적용합니다.";

// ── 해결 옵션 (정적) ──────────────────────────────────────────────────────────
pub const OPT_RESTORECON_DESC:     &str = "먼저 시도해 보세요. 기본 파일 컨텍스트를 복원합니다 (레이블 손상 수정).";
pub const OPT_CUSTOM_POLICY_LABEL: &str = "커스텀 정책 모듈 생성 및 적용 (audit2allow)";
pub const OPT_CUSTOM_POLICY_DESC:  &str = "audit2allow로 정책을 자동 생성합니다. 경로를 알고 있다면 먼저 P 키를 시도해 보세요.";
pub const OPT_PERMISSIVE_DESC:     &str = "⚠ 이 도메인의 모든 거부를 비활성화합니다. 보안이 크게 저하됩니다. 조사 목적으로만 사용하세요.";
pub const OPT_IGNORE_LABEL:        &str = "무시 / 무시 목록에 추가";
pub const OPT_IGNORE_DESC:         &str = "이 항목을 무시 목록에 추가합니다 (도구 내부에서만 적용).";

// ── 분석 (정적) ───────────────────────────────────────────────────────────────
pub const ANALYSIS_FCONTEXT_NONSTANDARD: &str = " 비표준 경로이므로 fcontext 규칙 추가가 필요합니다.";
pub const ANALYSIS_RESTORECON_FIX:       &str = " restorecon으로 기본 컨텍스트를 복원하면 해결될 수 있습니다.";
pub const ANALYSIS_CUSTOMPOLICY_FIX:     &str = " audit2allow로 커스텀 정책을 생성해야 합니다.";
pub const ANALYSIS_PATH_UNKNOWN_HINT: &str = " ※ 경로를 알 수 없습니다. P 키를 눌러 경로를 지정하면 최적의 수정 방법이 표시됩니다.";
pub const PATH_INPUT_TITLE:  &str = " 디렉터리 경로 입력";
pub const PATH_INPUT_PROMPT: &str = " 절대 경로를 입력하세요 (예: /var/log/myapp)";
pub const PATH_INPUT_HINT:   &str = " Enter: 확인  Esc: 취소";
pub const OPT_PATH_INPUT_LABEL: &str = "절대 경로를 입력하여 restorecon/fcontext 활성화";
pub const OPT_PATH_INPUT_DESC:  &str = "경로를 알 수 없어 A/B 수정 옵션을 표시할 수 없습니다. 절대 경로를 입력하면 레이블 수정 단계(restorecon / semanage fcontext)가 표시됩니다.";

// ── Remedy 표시 이름 ──────────────────────────────────────────────────────────
pub const REMEDY_PORT_CONTEXT:  &str = "포트 컨텍스트";
pub const REMEDY_FILE_CONTEXT:  &str = "파일 컨텍스트";
pub const REMEDY_RESTORECON:    &str = "restorecon";
pub const REMEDY_CUSTOM_POLICY: &str = "커스텀 정책";

// ── 형식 문자열 ───────────────────────────────────────────────────────────────
pub fn avc_list_title(unresolved: usize, total: usize) -> String {
    format!(" 접근 거부 목록  [오늘]  미처리: {}건 / 전체: {}건 ", unresolved, total)
}
pub fn module_list_title(count: usize) -> String {
    format!(" 정책 모듈 목록  {} 개 ", count)
}
pub fn module_delete_desc(name: &str) -> String {
    format!("정책 모듈 '{}' 를 삭제합니다.", name)
}
pub fn module_deleted(name: &str) -> String {
    format!("모듈 '{}' 가 삭제되었습니다.", name)
}
pub fn avc_loaded(count: usize) -> String {
    format!("AVC 항목 {}개 로드됨", count)
}
pub fn cmd_failed(first_line: &str) -> String {
    format!("명령 실패: {}", first_line)
}
pub fn lockout_msg(secs: u64) -> String {
    format!("  잠금 중 ({}초 후 해제)", secs)
}
pub fn log_overlay_title(total: usize) -> String {
    format!(" 로그  {}개  ↑↓:스크롤  Esc:닫기 ", total)
}
pub fn opt_port_label(proto: &str, port: &str) -> String {
    format!("포트 컨텍스트 추가  semanage port -a -t ssh_port_t -p {} {}", proto, port)
}
pub fn opt_port_desc(proto: &str, target: &str) -> String {
    format!("{} 포트 {}에 ssh_port_t 컨텍스트를 할당합니다.", proto, target)
}
pub fn opt_restorecon_label(path: &str) -> String {
    format!("restorecon으로 복원  restorecon -Rv {}", path)
}
pub fn opt_fcontext_label(file_type: &str, path: &str) -> String {
    format!("fcontext 변경 + restorecon  semanage fcontext -a -t '{}' '{}(/.*)?'", file_type, path)
}
pub fn opt_fcontext_desc(file_type: &str) -> String {
    format!("이 경로에 {}를 할당하는 규칙을 추가하고 restorecon을 자동 실행합니다.", file_type)
}
pub fn opt_bool_temp_label(bool_name: &str) -> String {
    format!("Boolean 활성화 (임시)  setsebool {} on", bool_name)
}
pub fn opt_bool_temp_desc(bool_name: &str) -> String {
    format!("{} 활성화 (재부팅 후 초기화됩니다).", bool_name)
}
pub fn opt_bool_perm_label(bool_name: &str) -> String {
    format!("Boolean 활성화 (영구)  setsebool -P {} on", bool_name)
}
pub fn opt_bool_perm_desc(bool_name: &str) -> String {
    format!("{} 영구 활성화.", bool_name)
}
pub fn opt_permissive_label(domain: &str) -> String {
    format!("도메인을 Permissive로 설정 (조사용 전용) ⚠  semanage permissive -a {}", domain)
}
pub fn analysis_denied(process: &str, target: &str, perm: &str) -> String {
    format!(" {}이(가) {} 에 대한 {}을(를) 거부당했습니다.", process, target, perm)
}
pub fn analysis_port_undefined(target: &str) -> String {
    format!(" 포트 {}은(는) SELinux 정책에 정의되어 있지 않습니다.", target)
}
pub fn analysis_port_nonstandard(process: &str) -> String {
    format!(" {}이(가) 비표준 포트에서 동작하려면 포트 컨텍스트 추가가 필요합니다.", process)
}
pub fn analysis_write_denied(target: &str) -> String {
    format!(" {}에 대한 쓰기 접근이 거부되었습니다.", target)
}
pub fn analysis_label_stripped(target: &str) -> String {
    format!(" {}의 레이블이 손실되었을 수 있습니다.", target)
}
pub fn analysis_dir_label_check(dir: &str) -> String {
    format!(" ls -dZ {} 로 디렉터리 레이블을 확인하세요. 레이블이 잘못되었다면 restorecon을 먼저 시도하세요.", dir)
}
pub fn analysis_bool_enable(b: &str) -> String {
    format!(" {} Boolean을 활성화하면 해결될 수 있습니다.", b)
}
pub fn analysis_domain_denied(domain: &str, perm: &str) -> String {
    format!(" 도메인 {}에서의 {} 작업은 정책에서 허용되지 않습니다.", domain, perm)
}
pub fn remedy_boolean(b: &str) -> String { format!("Boolean: {}", b) }
pub fn elapsed_secs(n: u64)  -> String { format!("{}초 전", n) }
pub fn elapsed_mins(n: u64)  -> String { format!("{}분 전", n) }
pub fn elapsed_hours(n: u64) -> String { format!("{}시간 전", n) }
pub fn elapsed_days(n: u64)  -> String { format!("{}일 전", n) }
pub const LABEL_FIRST_SEEN: &str = "최초 발생";
pub const LABEL_LAST_SEEN:  &str = "최종 발생";
pub const LABEL_SYSCALL:    &str = "Syscall";
pub const LABEL_ERRNO:      &str = "errno";
pub fn warn_locale_not_utf8(lang_val: &str) -> String {
    format!(
        "경고: 로케일이 UTF-8이 아닐 수 있습니다 (LANG={}).\n\
         문자가 올바르게 표시되지 않으면 LANG=ko_KR.UTF-8을 설정하세요.",
        lang_val
    )
}

// ── 의존성 검사 출력 ──────────────────────────────────────────────────────────
pub const WARN_MISSING_OPT_HDR: &str = "[WARN] 일부 기능에 필요한 명령을 찾을 수 없습니다:";
pub const WARN_MISSING_OPT_FTR: &str = "       위 명령을 사용하는 기능은 작동하지 않습니다.";
pub const ERR_MISSING_CRIT_HDR: &str = "[ERROR] 필수 명령을 찾을 수 없습니다. seadmin을 시작할 수 없습니다:";
pub fn warn_missing_cmd(cmd: &str, pkg: &str) -> String {
    format!("  {:<14} (패키지: {})", cmd, pkg)
}
pub const ERR_INSTALL_HINT: &str = "\
위 패키지를 설치한 후 다시 실행하십시오.\n\
  예 (Fedora/RHEL):   sudo dnf install audit policycoreutils\n\
  예 (Debian/Ubuntu): sudo apt install auditd policycoreutils";

// ── 로그 출력 ─────────────────────────────────────────────────────────────────
pub fn log_startup(path: &str) -> String { format!("[INFO] seadmin 시작 (log: {})", path) }
pub fn log_file_open_error(err: &str) -> String { format!("[WARN] 로그 파일을 열 수 없습니다: {}", err) }
pub fn log_avc_loaded_n(count: usize) -> String { format!("[INFO] AVC 로드: {}개", count) }
pub fn log_path_no_abs(target: &str) -> String {
    format!("path={} (절대 경로 없음 — restorecon/fcontext 숨김)", target)
}
pub fn log_avc_load_error(err: &str) -> String { format!("[ERR] AVC 로드 실패: {}", err) }
pub const LOG_CMD_OK: &str = "[OK] 명령 성공";
pub fn log_auth_failed(n: u32) -> String { format!("[ERR] 인증 실패 ({}/3)", n) }
pub fn log_cmd_failed_msg(stderr: &str) -> String { format!("[ERR] 명령 실패:\n{}", stderr) }
pub fn log_selinux_mode(mode: &str) -> String { format!("[INFO] SELinux 모드: {}", mode) }
pub fn log_audit2allow_done(lines: usize, pp: &str) -> String {
    format!("[INFO] audit2allow 생성 완료: {}줄, pp={}", lines, pp)
}
pub fn log_audit2allow_cmd(module: &str, count: usize) -> String {
    format!("[CMD] audit2allow -M {} ({}줄 로그 입력)", module, count)
}
pub fn log_sudo_cached(cmd: &str) -> String { format!("[CMD] sudo {} (캐시 인증)", cmd) }

// ── 명령 오류 ─────────────────────────────────────────────────────────────────
pub const ERR_AUDIT_NO_PERM: &str =
    "audit.log를 읽을 권한이 없습니다. adm 그룹에 추가하거나 sudo를 설정하십시오.";
pub fn err_audit2allow_failed(stderr: &str) -> String { format!("audit2allow 실패: {}", stderr) }
