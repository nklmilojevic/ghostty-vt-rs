// Written by .github/workflows/prebuilt.yml from the release `ghostty-<commit12>`: the Ghostty
// commit the archives were built from, and the SHA-256 of each target's archive. A commit other
// than build.rs's GHOSTTY_COMMIT means no prebuilt library for any target.
const PREBUILT_COMMIT: &str = "4a4fc017940bec38c93b278c10c6a18c591f3e63";
const PREBUILT: &[(&str, &str)] = &[
    ("aarch64-apple-darwin", "6e2a4196776954fc409e3cbf5903d4886c5186efb7d43ce1b3d37b187d18b36c"),
    ("aarch64-apple-ios-sim", "e154970f017655a396abd9680b5c9e304d2179200cf3ea8cec70fffcf5b42cf8"),
    ("aarch64-apple-ios", "864a0ed490b692266ac0b66b3b46c2cf09461a044bf26ce7ff4796532c657469"),
    ("aarch64-unknown-linux-gnu", "d139ff8f1431f83c74789109a187ea511792450639b7676d317c9c8dd46bc41b"),
    ("aarch64-unknown-linux-musl", "ae905105773186be1a2bfd43b5fff5e0c7b7cd3d582c17396b7e5fb638364c2b"),
    ("wasm32-unknown-unknown", "47309773d8fbc21f2a282ad6c09dc698b619683e85357c02130da2254098bc21"),
    ("x86_64-unknown-linux-gnu", "54f1b939ed1cce662d20e3d58829bdf4c15ee2b707785318d7cd912166227c8a"),
    ("x86_64-unknown-linux-musl", "48786bd9ca915575a38337ab3c0ade536743b8ae2b9f4eab55708533aecd977e"),
];
