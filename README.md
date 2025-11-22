# SF-Zhou's Blog [![Deploy](https://github.com/SF-Zhou/sf-zhou.github.io/workflows/Deploy/badge.svg)](https://github.com/SF-Zhou/sf-zhou.github.io/actions)

> A static blog based on GitHub Pages, GitHub Actions and Node.js.

## Build

Node 10 or later is required.

``` bash
# install dependencies
npm install

# generate site
npm run build
```

## RSS Feed

The build process automatically generates an RSS feed at `public/rss.xml` containing the latest 20 articles. The RSS feed URL is: https://sf-zhou.github.io/rss.xml
