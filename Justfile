# Cut a release with vesnik, after checking the pins against the prebuilt archives.
release:
    scripts/check-pins.sh --release
    vesnik release --yes
