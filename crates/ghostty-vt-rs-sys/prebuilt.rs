// Written by .github/workflows/prebuilt.yml from the release `ghostty-<commit12>`: the Ghostty
// commit the archives were built from, and the SHA-256 of each target's archive. A commit other
// than build.rs's GHOSTTY_COMMIT means no prebuilt library for any target.
const PREBUILT_COMMIT: &str = "1e9e14af5df14211892f5fc15fd21a49c7d94868";
const PREBUILT: &[(&str, &str)] = &[
    ("aarch64-apple-darwin", "58c2d5ed1cfedbc4c5093717f16a8dc6edad4f27c1a3661ba09fefbf3c526270"),
    ("aarch64-apple-ios-sim", "aeb07a80056a33af4343566570e7c9c81e98a953a319d03732bb5912b76c1469"),
    ("aarch64-apple-ios", "f1fd16d07a9ab5af5f41057ef0eeb09e5de07d01ea0008f69412697aa9077e04"),
    ("aarch64-unknown-linux-gnu", "73369271de63054a07b575a5020ff43b827af61626fb1531a9f20475827c67f3"),
    ("aarch64-unknown-linux-musl", "67a3f31fadddaad3ec441820e27b816651142bfbabbebdfe75b264c2d5915ba3"),
    ("wasm32-unknown-unknown", "a0f3d54307d315f438264eb4977280fb56b0a35bc2b19425344749fa859dbef7"),
    ("x86_64-unknown-linux-gnu", "5106c2a58491733f589f04d2af1401e22d32947fd7aa11d1719b5c26bfe529ce"),
    ("x86_64-unknown-linux-musl", "956299f95cc4537a69a6dd57b4f287943c8642ab0015ebadcb74a22cea64f77d"),
];
