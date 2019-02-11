#!/bin/sh

mkdir -p public
node src/main.js
cp -r posts/images public
cp -r dist public

# Less to CSS
./node_modules/.bin/lessc dist/main.less public/dist/main.css

# Highlight JS
cp ./node_modules/highlight.js/styles/github.css public/dist

# KaTeX
cp ./node_modules/katex/dist/katex.min.css public/dist
cp ./node_modules/katex/dist/katex.min.js public/dist
cp -r ./node_modules/katex/dist/fonts public/dist

# Gitalk
sed /sourceMappingURL=/d ./node_modules/gitalk/dist/gitalk.css > public/dist/gitalk.css
sed /sourceMappingURL=/d ./node_modules/gitalk/dist/gitalk.min.js > public/dist/gitalk.min.js
