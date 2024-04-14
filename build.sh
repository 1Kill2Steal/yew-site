#!/bin/bash

set -e # Exit the script if any builds fail.

echo "Setting up the rustup toolchain"
rustup toolchain install nightly
rustup default nightly

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown

echo "Installing cargo make"
cargo install cargo-make

echo "Running first test"
cargo make test_1
echo "Setting up the JSON files"
cargo make 1

echo "Checking if trunk is installed"
if ! which trunk >/dev/null 2>&1; then
	echo "Installing trunk"
	cargo install trunk
else
	echo "Trunk is already installed"
fi

# echo "Installing wasm-pack"
# cargo install wasm-pack

echo "Building site"
trunk build --release
