// Written by .github/workflows/prebuilt.yml from the release `ghostty-<commit12>`: the Ghostty
// commit the archives were built from, and the SHA-256 of each target's archive. A commit other
// than build.rs's GHOSTTY_COMMIT means no prebuilt library for any target.
const PREBUILT_COMMIT: &str = "0fa3d577a5614150ff8e2c152de5fb9d5c6d2dec";
const PREBUILT: &[(&str, &str)] = &[
    ("aarch64-apple-darwin", "9ad9a7e52b2ddd389c01315a31d4ef5d212dfca95617559dd95a9de8b898cc86"),
    ("aarch64-apple-ios-sim", "d51e8c6bf2901761ceb0bfa7735a6330027e467356afd6f2d0e9ef642bfb1746"),
    ("aarch64-apple-ios", "800e9352edbee3df60fb69ddb5ca6c99b22e8e86c71212f6d76340308543d482"),
    ("aarch64-unknown-linux-gnu", "7075b6638d0d20e715fa10835a1764d3f9ccc21a08fe3c82dfd498a656782a18"),
    ("aarch64-unknown-linux-musl", "d2012a4423f43a2658cf9268105ba3db0c3d5a7f76f032f00d08857a5215696c"),
    ("wasm32-unknown-unknown", "d0d34afa0a0e26d1cb33f4e11b18db65f2f43c1ad60a0ef56d6b6910b2a73621"),
    ("x86_64-unknown-linux-gnu", "84021c0d0944e38883eb5ffda6241653d24aa9b533af4fbc0e64c1433ea0e5ef"),
    ("x86_64-unknown-linux-musl", "4810fbcc2ce6ad2be99d3e2e189ebfedaba56005944e23db990e6b61b9c46c9f"),
];
