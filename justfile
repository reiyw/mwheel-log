lint:
    cargo clippy -- -D clippy::all -W clippy::nursery
    cargo +nightly fmt -- --check
