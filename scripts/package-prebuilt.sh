#!/usr/bin/env bash
# Builds libghostty-vt from source for TARGET (Zig 0.17 on PATH) and packs the install prefix the
# crate links against: lib/libghostty-vt.a, include/ and Ghostty's license.
#   scripts/package-prebuilt.sh TARGET OUT_DIR
# Writes OUT_DIR/ghostty-vt-<commit12>-<TARGET>.tar.gz and its .sha256.
set -euo pipefail

target="${1:?target}"
out="$(mkdir -p "${2:?out dir}" && cd "$2" && pwd)"
repo="$(cd "$(dirname "$0")/.." && pwd)"
crate="$repo/crates/ghostty-vt-rs-sys"
commit="$(sed -n 's/^const GHOSTTY_COMMIT: &str = "\([0-9a-f]*\)";/\1/p' "$crate/build.rs")"
[[ ${#commit} -eq 40 ]] || { echo "no GHOSTTY_COMMIT in build.rs" >&2; exit 1; }
name="ghostty-vt-${commit:0:12}-$target.tar.gz"

cd "$repo"
# The shipped settings: `fast` (ReleaseFast) and the default CPU, whatever the environment says.
unset GHOSTTY_VT_RS_SYS_CPU GHOSTTY_VT_RS_SYS_OPTIMIZE GHOSTTY_SOURCE_DIR GHOSTTY_ZIG_SYSTEM_DIR
prefix="$(GHOSTTY_VT_RS_SYS_BUILD=source cargo build --release --locked --target "$target" \
    -p ghostty-vt-rs-sys --message-format=json |
    jq -r 'select(.reason == "build-script-executed" and (.package_id | contains("ghostty-vt-rs-sys"))) | .out_dir' |
    tail -n1)/ghostty-install"
[[ -f "$prefix/lib/libghostty-vt.a" && -f "$prefix/include/ghostty/vt.h" ]] ||
    { echo "no libghostty-vt.a or headers under $prefix" >&2; exit 1; }

stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/lib"
cp "$prefix/lib/libghostty-vt.a" "$stage/lib/"
cp -R "$prefix/include" "$stage/include"
cp "$crate/LICENSE-ghostty" "$stage/LICENSE-ghostty"
# Files in a fixed order, with GNU or BSD tar.
(cd "$stage" && find . -type f | LC_ALL=C sort | tar -czf "$out/$name" -T -)
(cd "$out" && { sha256sum "$name" 2>/dev/null || shasum -a 256 "$name"; } > "$name.sha256")
cat "$out/$name.sha256"
