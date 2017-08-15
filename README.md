# SF-Zhou's Blog

[![wercker status](https://app.wercker.com/status/94144b91388fbf8712fca882f24eb63e/s/blog "wercker status")](https://app.wercker.com/project/byKey/94144b91388fbf8712fca882f24eb63e)

> A Blog based on GitHub Pages, Wercker, Vue.js, Node.js & Element.

## Generate Steps

Node 7.6.0 or later is required, or using babel for async/await support.

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
- [x] Deploy on GitHub Pages by Wercker;
- [x] Set custom domain;
- [x] Render a set of MarkDown articles;
- [x] Support code highlight;
- [x] Support LaTeX mathematics formula;
- [x] Add date, category and author info reading;
- [x] Generate site index.html page;
- [x] Support tag search;
- [ ] Generate category index.html pages;
- [ ] Support global text search;
- [ ] Support comment with RESTful API;
- [ ] Support comment @ notification;
