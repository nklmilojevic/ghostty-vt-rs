# ghostty-vt-rs

Rust bindings for libghostty-vt, the terminal emulation library of [Ghostty](https://ghostty.org).

- [`ghostty-vt-rs-sys`](crates/ghostty-vt-rs-sys): raw FFI bindings, with prebuilt static
  libraries for macOS, Linux (glibc and musl), iOS and wasm32, so no Zig is needed to build.

MIT licensed; see `crates/ghostty-vt-rs-sys/LICENSE`.
