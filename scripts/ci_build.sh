# Check person-lambda
cd person-lambda &&
cargo fmt --all -- --check &&
cargo clippy --all-targets -- -D clippy::all &&
cargo check --all
cargo build
cargo test