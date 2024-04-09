#!/bin/bash

echo "Setting up the rustup toolchain"
rustup toolchain install nightly
rustup default nightly

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown

echo "Installing trunk"
cargo install trunk

echo "Building site"
trunk build --release