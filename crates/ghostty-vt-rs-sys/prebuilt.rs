// Written by .github/workflows/prebuilt.yml from the release `ghostty-<commit12>`: the Ghostty
// commit the archives were built from, and the SHA-256 of each target's archive. A commit other
// than build.rs's GHOSTTY_COMMIT means no prebuilt library for any target.
const PREBUILT_COMMIT: &str = "573c4a21e02311ae87844af26b6a7a179f82858e";
const PREBUILT: &[(&str, &str)] = &[
    ("aarch64-apple-darwin", "138eaa723c92c444203f994914d6ddee5994f60d72d2b357e1ea39b4f70a3e16"),
    ("aarch64-apple-ios-sim", "563e6a374652df6b8e68743a9b8c9b7e42f8a6e13621d3f12cda56d89afb3083"),
    ("aarch64-apple-ios", "543e09968d04aaa946313ecab26f7587769ab8d76863bd1f0fd203617954650a"),
    ("aarch64-unknown-linux-gnu", "8b9e3b8507cb75f75bdb42b34e45fbb38bd0da8f401b8000f1c2a4a0fc412125"),
    ("aarch64-unknown-linux-musl", "7861ebece396d7476b4908696277b2db0e827ce5b7e81f9c10e5180cd613fa52"),
    ("wasm32-unknown-unknown", "6c36eb0a97c853ffd541ebac4a76e42d99169bf2c591550659113fcf3599a232"),
    ("x86_64-unknown-linux-gnu", "69ce162abb4fff54135489a2e970a3de7ca5cdcd5b27f97d537a39c2242b4d6e"),
    ("x86_64-unknown-linux-musl", "abd7a244c82526c493649430f3ca4123047205079db008a3203eb347626c9f9e"),
];
