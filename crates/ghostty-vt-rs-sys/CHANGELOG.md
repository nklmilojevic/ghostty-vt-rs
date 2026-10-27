# Changelog

## v0.2.0 - 2026-10-27

This release moves the crate to a newer Ghostty version with kitty graphics and search APIs. It needs Zig 0.17 and removes two bindings, so existing code might break.

### New features

- The crate now builds Ghostty 4a4fc01 and requires Zig 0.17. Code that uses `ghostty_terminal_vt_write_until_ground` or `OscCommandType::CONTEXT_SIGNAL` must change to `ghostty_terminal_vt_write_until` and `OscCommandType::HIERARCHICAL_CONTEXT`.

### Other changes

- Prebuilt libghostty-vt archives are available for Ghostty 4a4fc01.
