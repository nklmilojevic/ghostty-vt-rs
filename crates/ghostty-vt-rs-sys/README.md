# ghostty-vt-rs-sys

Raw FFI bindings for **libghostty-vt**, the terminal emulation library of
[Ghostty](https://ghostty.org), with **prebuilt static libraries**: no Zig needed for the
common targets.

```toml
[dependencies]
ghostty-vt-rs-sys = "0.1"
```

The bindings (`src/bindings.rs`) are generated from the headers of the pinned Ghostty commit
(`[package.metadata.ghostty] commit` in `Cargo.toml`). libghostty-vt is pre-1.0: each version
of this crate follows one Ghostty commit.

## How the library is found

1. **`pkg-config` feature**: an installed `libghostty-vt-static` (or `libghostty-vt` with
   `link-dynamic`).
2. **Prebuilt** (the default): a static library for the target, downloaded once from this
   repository's release `ghostty-<commit12>` with `curl` and checked against the SHA-256 pinned
   in `prebuilt.rs`. Built ReleaseFast for every profile, for the target's default CPU.
   Targets: `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`,
   `aarch64-unknown-linux-gnu`, `aarch64-unknown-linux-musl`, `aarch64-apple-ios`,
   `aarch64-apple-ios-sim`.
3. **From source**: Ghostty at the pinned commit, built with Zig 0.16. Used for any other
   target, for `link-dynamic`, and when asked for below.

| Variable | Effect |
|---|---|
| `GHOSTTY_VT_RS_SYS_BUILD` | `prebuilt` (fail without one) or `source` (always build with Zig) |
| `GHOSTTY_VT_RS_SYS_PREBUILT_DIR` | An unpacked prebuilt archive to use as is, e.g. fetched by Nix outside its sandbox |
| `GHOSTTY_VT_RS_SYS_PREBUILT_URL` | Another base URL for the archives (a mirror) |
| `GHOSTTY_VT_RS_SYS_CACHE_DIR` | Kept across builds: downloaded archives, the Ghostty checkout and Zig's cache |
| `GHOSTTY_SOURCE_DIR` | A Ghostty checkout to build from (source build) |
| `GHOSTTY_ZIG_SYSTEM_DIR` | Ghostty's Zig packages, fetched ahead (offline source build) |
| `GHOSTTY_VT_RS_SYS_CPU`, `GHOSTTY_VT_RS_SYS_OPTIMIZE` | Zig's `-Dcpu` and optimize mode (source build) |

A prebuilt archive holds `lib/libghostty-vt.a`, `include/ghostty/` and Ghostty's license
(`LICENSE-ghostty`). Each one's SHA-256 is pinned in `prebuilt.rs`, so a build only links the
archive this crate version was published with.

## Moving to another Ghostty commit

1. Set `GHOSTTY_COMMIT` in `build.rs` and `commit` in `Cargo.toml`.
2. Build from source once (`GHOSTTY_VT_RS_SYS_BUILD=source cargo build`, Zig 0.16 on `PATH`) and
   regenerate the bindings from the installed headers:
   `GHOSTTY_INCLUDE_DIR=<target>/debug/build/ghostty-vt-rs-sys-<hash>/out/ghostty-install/include cargo run --features bindgen-tool --bin gen-bindings`.
3. Merge, then run the **Prebuilt** workflow: it publishes the archives as the release
   `ghostty-<commit12>` and opens a pull request with their checksums in `prebuilt.rs`.
4. Merge that, bump the version, and tag `vX.Y.Z`: the **Publish** workflow publishes to
   crates.io.

## Credits

Started from [`libghostty-vt-sys`](https://github.com/Uzaaft/libghostty-rs) by Uzair Aftab and
Leah Amelia Chen (MIT), with the Apple CPU default, a newer Ghostty, a cache directory and the
prebuilt libraries added. Ghostty and libghostty-vt are by Mitchell Hashimoto and the Ghostty
contributors (MIT, `LICENSE-ghostty`).
