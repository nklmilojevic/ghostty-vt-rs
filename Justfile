# Cut a release with vesnik, after checking the pins against the prebuilt archives.
release:
    @test "$(git branch --show-current)" = main || { echo "release from main" >&2; exit 1; }
    @test -z "$(git status --porcelain)" || { echo "release from a clean tree: vesnik commits untracked files it writes to, such as CHANGELOG.md" >&2; git status --short >&2; exit 1; }
    git pull --ff-only origin main
    scripts/check-pins.sh --release
    vesnik release --yes
