const hljs = require('highlight.js');
const kt = require('katex');
const tm = require('markdown-it-texmath').use(kt);
const mi = require('markdown-it-linkify-images');
const markdown_it = require('markdown-it');

const md = markdown_it({
  linkify: true,
  typography: true,
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      return `<pre class="hljs"><code>${hljs.highlight(lang, str).value}</code></pre>`;
    } else {
      return `<pre class="hljs"><code>${hljs.highlightAuto(str).value}</code></pre>`;
    }
  }
});
md.use(tm);
md.use(mi);

module.exports = (markdown) => {
  return md.render(markdown);
}
