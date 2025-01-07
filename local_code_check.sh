#!/bin/bash

# exit when any command fails
set -e

cargo fmt --all -- --check
cargo clippy --workspace --bins --examples --tests --no-deps
cargo test --release --workspace
cargo doc
