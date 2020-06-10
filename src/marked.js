const prism = require('prismjs');
const katex = require('katex');
const texmath = require('markdown-it-texmath').use(katex);
const implicit_figures = require('markdown-it-implicit-figures');
const anchor = require('markdown-it-anchor');
const markdown_it = require('markdown-it');

const load_languages = require('prismjs/components/');
load_languages(['bash', 'cmake', 'cpp', 'json', 'nasm', 'protobuf', 'python', 'rust', 'yaml']);

const md = markdown_it({
  linkify: true,
  typography: true,
  highlight: function (str, lang) {
    lang = lang.toLowerCase();
    const map = {
      '': 'markup',
      'c++': 'cpp',
      'yml': 'yaml',
      'asm': 'nasm',
      'assembly': 'nasm',
    }
    if (lang in map) {
      lang = map[lang];
    }

    let hl;
    try {
      hl = prism.highlight(str, prism.languages[lang]);
    } catch (error) {
      console.log(lang, error);
      hl = md.utils.escapeHtml(str)
    }
    return `<pre class="language-${lang}"><code class="language-${lang}">${hl}</code></pre>`;
  }
});
md.use(implicit_figures, {
  dataType: false,
  figcaption: true,
  tabindex: true,
  link: true,
})
md.use(texmath);
md.use(anchor, {
  permalink: true,
  slugify: s => String(s).trim().toLowerCase().replace(/\s+/g, '-'),
  renderPermalink: (slug, opts, state, idx) => {
    let children = state.tokens[idx + 1].children;
    let child = children[0];
    let level = child.level;
    child.level = level + 1;

    token = new state.Token('link_open', 'a', 1);
    token.attrs = [['href', opts.permalinkHref(encodeURIComponent(slug), state)]];
    token.level = level;
    children.unshift(token);

    token = new state.Token('link_close', 'a', -1);
    token.level = level;
    children.push(token);
  }
});

module.exports = (markdown) => {
  return md.render(markdown);
}
