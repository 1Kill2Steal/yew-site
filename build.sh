#!/bin/bash

echo "Adding dirs from index.html data trunks"
mkdir hutao
mkdir hutao/pics
mkdir hutao/pics_uncompressed

echo "Setting up the rustup toolchain"
rustup toolchain install nightly
rustup default nightly

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown

echo "Installing trunk"
cargo install trunk

echo "Building site"
trunk build --release