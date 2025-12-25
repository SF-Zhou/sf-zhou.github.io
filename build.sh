#!/bin/sh
set -e

mkdir -p public
mkdir -p profile

# Build Rust generator if needed
if [ ! -f ./target/release/blog-generator ]; then
    echo "Building Rust blog generator..."
    cargo build --release
fi

# Build blog using Rust generator
./target/release/blog-generator --config config.toml --templates src

# Copy static assets
cp -r posts/images public
cp -r dist public
