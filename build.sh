#!/bin/sh
set -e

mkdir -p public
mkdir -p profile
node src/main.js
cp -r posts/images public
cp -r dist public

# Less to CSS
npx lessc dist/main.less public/dist/main.css

# Highlight JS
cp ./node_modules/prismjs/themes/prism-tomorrow.css public/dist/highlight.css

# KaTeX
cp ./node_modules/katex/dist/katex.min.css public/dist
cp -r ./node_modules/katex/dist/fonts public/dist
