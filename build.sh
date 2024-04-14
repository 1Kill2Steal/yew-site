#!/bin/bash

set -e # Exit the script if any builds fail.

echo "Setting up the rustup toolchain"
rustup toolchain install nightly &&
	rustup default nightly

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown &

echo "Installing cargo make && trunk"
if ! which trunk >/dev/null 2>&1; then
	echo "Installing trunk"
	cargo install trunk
else
	echo "Trunk is already installed"
fi
if ! which cargo make >/dev/null 2>&1; then
	echo "Installing cargo make"
	cargo install cargo-make
else
	echo "Cargo make is already installed"
fi

echo "Running first tests && utils"
cargo make test_1 &&
	cargo make 1

echo "Building site"
trunk build --release
