# SF-Zhou's Blog [![Build Status](https://travis-ci.com/SF-Zhou/sf-zhou.github.io.svg?branch=blog)](https://travis-ci.com/SF-Zhou/sf-zhou.github.io)

> A Blog based on GitHub Pages, Travis CI, Vue.js, Node.js & Element.

## Generate Steps

Node 7.6.0 or later is required.

``` bash
# install dependencies
npm install

# generate static html pages
npm run gene

# serve with hot reload at localhost:8080
npm run dev

# generate final site
npm run build
```

## Roadmap

- [x] Design article page;
- [x] Render simple MarkDown article;
- [x] Deploy on GitHub Pages by Travis CI;
- [x] Set custom domain;
- [x] Render a set of MarkDown articles;
- [x] Support code highlight;
- [x] Support LaTeX mathematics formula;
- [x] Add date, category and author info reading;
- [x] Generate site index.html page;
- [x] Support tag search;
- [ ] Generate category index.html pages;
- [x] Support global text search (Google with 'something site:sf-zhou.github.io');
- [x] Support comment with RESTful API (Gitalk);
- [x] Support comment @ notification (Gitalk);
