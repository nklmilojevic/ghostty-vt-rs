#!/usr/bin/env bash
# The Ghostty commit appears in build.rs (GHOSTTY_COMMIT) and Cargo.toml
# ([package.metadata.ghostty] commit); prebuilt.rs names the commit its archives were built from.
#   scripts/check-pins.sh [--release]
# --release also requires prebuilt.rs to cover that commit (publishing without archives would
# make every user build from source).
set -euo pipefail
crate="$(cd "$(dirname "$0")/.." && pwd)/crates/ghostty-vt-rs-sys"
build_rs="$(sed -n 's/^const GHOSTTY_COMMIT: &str = "\([0-9a-f]*\)";/\1/p' "$crate/build.rs")"
cargo_toml="$(sed -n 's/^commit = "\([0-9a-f]*\)"/\1/p' "$crate/Cargo.toml")"
prebuilt="$(sed -n 's/^const PREBUILT_COMMIT: &str = "\([0-9a-f]*\)";/\1/p' "$crate/prebuilt.rs")"
echo "build.rs: $build_rs"
echo "Cargo.toml: $cargo_toml"
echo "prebuilt.rs: ${prebuilt:-none}"
[[ ${#build_rs} -eq 40 && "$build_rs" == "$cargo_toml" ]] ||
    { echo "::error::build.rs and Cargo.toml pin different Ghostty commits"; exit 1; }
if [[ "$prebuilt" != "$build_rs" ]]; then
    if [[ "${1:-}" == --release ]]; then
        echo "::error::prebuilt.rs has no archives for $build_rs: run the Prebuilt workflow first"
        exit 1
    fi
    echo "::warning::prebuilt.rs has no archives for $build_rs yet: builds use Zig until the Prebuilt workflow's PR is merged"
fi
