# Changelog

## v0.3.0 - 2026-11-07

This release builds the bindings from libghostty-vt 1.0.0, tag vt-1.0.0, the first release with a stable C ABI. The API changed, and the scrollback limit now defaults to 10,000 bytes, so update your code before you upgrade.

### New features

- Adds the `STYLE_FLAG_*`, `STYLE_UNDERLINE_MASK` and `IMAGE_LAYER_MASK_*` constants. The encoder, formatter, style, cursor and Kitty image APIs change, so update your calls.

## v0.2.0 - 2026-10-27

This release moves the crate to a newer Ghostty version with kitty graphics and search APIs. It needs Zig 0.17 and removes two bindings, so existing code might break.

### New features

- The crate now builds Ghostty 4a4fc01 and requires Zig 0.17. Code that uses `ghostty_terminal_vt_write_until_ground` or `OscCommandType::CONTEXT_SIGNAL` must change to `ghostty_terminal_vt_write_until` and `OscCommandType::HIERARCHICAL_CONTEXT`.

### Other changes

- Prebuilt libghostty-vt archives are available for Ghostty 4a4fc01.
