const prism = require('prismjs');
const katex = require('katex');
const texmath = require('markdown-it-texmath').use(katex);
const linkify_image = require('markdown-it-linkify-images');
const markdown_it = require('markdown-it');

var load_languages = require('prismjs/components/');
load_languages(['cpp', 'python', 'bash', 'json', 'yaml']);

const md = markdown_it({
  linkify: true,
  typography: true,
  highlight: function (str, lang) {
    lang = lang.toLowerCase();
    const map = {
      '': 'markup',
      'cmake': 'markup',
      'c++': 'cpp',
      'yml': 'yaml',
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
md.use(texmath);
md.use(linkify_image);

module.exports = (markdown) => {
  return md.render(markdown);
}
