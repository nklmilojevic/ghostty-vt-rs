//! The library links into a wasm module and runs there (`scripts/wasm-smoke.mjs` calls `smoke`).
//! Built as a cdylib for every target so `--all-targets` covers it; only wasm32 runs it.

use std::ffi::c_void;

use ghostty_vt_rs_sys as ffi;

/// Creates an 80x24 terminal, writes two lines and reads the cursor back. Returns
/// `col * 100 + row` (201 when everything works) or a negative step that failed.
#[unsafe(no_mangle)]
pub extern "C" fn smoke() -> i32 {
    let mut terminal: ffi::Terminal = std::ptr::null_mut();
    if unsafe { ffi::ghostty_terminal_new(std::ptr::null(), &mut terminal, 80, 24) }
        != ffi::Result::SUCCESS
    {
        return -1;
    }
    let bytes = b"\x1b[31mhello\r\nab";
    unsafe { ffi::ghostty_terminal_vt_write(terminal, bytes.as_ptr(), bytes.len()) };
    let (mut col, mut row) = (0u16, 0u16);
    let ok = unsafe {
        ffi::ghostty_terminal_get(
            terminal,
            ffi::TerminalData::CURSOR_X,
            (&raw mut col).cast::<c_void>(),
        ) == ffi::Result::SUCCESS
            && ffi::ghostty_terminal_get(
                terminal,
                ffi::TerminalData::CURSOR_Y,
                (&raw mut row).cast::<c_void>(),
            ) == ffi::Result::SUCCESS
    };
    unsafe { ffi::ghostty_terminal_free(terminal) };
    if !ok {
        return -2;
    }
    i32::from(col) * 100 + i32::from(row)
}
