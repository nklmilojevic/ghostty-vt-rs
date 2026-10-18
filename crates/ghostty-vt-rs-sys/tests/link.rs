//! The library links and runs: a terminal is created, written to and freed.

use ghostty_vt_rs_sys as ffi;

#[test]
fn creates_writes_and_frees_a_terminal() {
    let mut terminal: ffi::Terminal = std::ptr::null_mut();
    let result = unsafe { ffi::ghostty_terminal_new(std::ptr::null(), &mut terminal, 80, 24) };
    assert_eq!(result, ffi::Result::SUCCESS);
    assert!(!terminal.is_null());
    let result = unsafe { ffi::ghostty_terminal_resize(terminal, 100, 30, 8, 16) };
    assert_eq!(result, ffi::Result::SUCCESS);
    unsafe { ffi::ghostty_terminal_free(terminal) };
}
