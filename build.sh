#!/bin/bash

set -e # Exit the script if any builds fail.

echo "Setting up the rustup toolchain"
rustup toolchain install nightly
rustup default nightly

echo "Adding linux target"
rustup target add x86_64-unknown-linux-gnu

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown

echo "Installing cargo make"
cargo install cargo-make

echo "Running first test"
cargo make test_1
echo "Setting up the JSON files"
cargo make 1

echo "Installing trunk"
cargo install trunk

# echo "Installing wasm-pack"
# cargo install wasm-pack

echo "Building site"
trunk build --release
