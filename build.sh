#!/bin/bash

set -euo pipefail # Exit the script if any pipelines fail.

# Set some specific optimization flags for the rust compiler
# export RUSTFLAGS="-C opt-level=3 -C target-cpu=native"

# Setting up the rustup toolchain & cargo binaries
rustup toolchain install nightly &&
	rustup default nightly &
rustup target add wasm32-unknown-unknown &

cargo install cargo-make &
if ! command trunk >/dev/null 2>&1; then
	cargo install trunk &
fi

wait # To make sure cargo-make is installed.

# Running tests
cargo make test_1 &
cargo make 1 &

# Building the project
trunk build --release &

# Deploying
trunk serve --release
