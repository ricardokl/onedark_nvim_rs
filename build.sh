#!/bin/bash
set -e

# Build the Rust library
cargo build --release

# Copy the compiled library to the lua directory
\cp ./target/release/libonedark_nvim_rs.so /home/ricardo/.config/nvim/lua/onedark_nvim_rs.so

echo "Plugin built successfully"

