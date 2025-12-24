#!/bin/sh
set -e

mkdir -p public
mkdir -p profile

# Build Rust generator if needed
if [ ! -f ./blog-generator/target/release/blog-generator ]; then
    echo "Building Rust blog generator..."
    cd blog-generator && cargo build --release && cd ..
fi

# Build blog using Rust generator
./blog-generator/target/release/blog-generator --config config.toml --templates src

# Copy static assets
cp -r posts/images public
cp -r dist public

# Less to CSS
npx lessc dist/main.less public/dist/main.css

# Syntax highlighting CSS (using syntect, but still need the base styles)
# Note: Rust generator uses syntect for syntax highlighting, using prism theme for backward compatibility
cp ./node_modules/prismjs/themes/prism-tomorrow.css public/dist/highlight.css

# KaTeX CSS and fonts
cp ./node_modules/katex/dist/katex.min.css public/dist
cp -r ./node_modules/katex/dist/fonts public/dist
