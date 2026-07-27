import prismjs from 'prismjs';
const { highlight, languages } = prismjs;
import katex from 'katex';
import texmath from 'markdown-it-texmath';
import implicit_figures from 'markdown-it-implicit-figures';
import anchor from 'markdown-it-anchor';
import markdown_it from 'markdown-it';
import toc from 'markdown-it-table-of-contents';
import md5 from 'md5';

import { render_mermaid } from './mermaid.js';

import load_languages from 'prismjs/components/index.js';
load_languages(['bash', 'cmake', 'cpp', 'json', 'lua', 'nasm', 'protobuf', 'python', 'rust', 'toml', 'yaml']);

let mermaid_blocks = [];

const md = markdown_it({
  linkify: true,
  typography: true,
  highlight: function (str, lang) {
    lang = lang.toLowerCase();
    if (lang === 'mermaid') {
      const index = mermaid_blocks.length;
      mermaid_blocks.push(str);
      return `<pre class="mermaid-placeholder">${index}</pre>`;
    }
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
      hl = highlight(str, languages[lang]);
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
md.use(texmath.use(katex));
md.use(anchor, {
  permalink: true,
  slugify: s => String(s).trim().toLowerCase().replace(/\s+/g, '-'),
  renderPermalink: (slug, opts, state, idx) => {
    let children = state.tokens[idx + 1].children;
    let child = children[0];
    let level = child.level;
    child.level = level + 1;

    let token = new state.Token('link_open', 'a', 1);
    token.attrs = [['href', opts.permalinkHref(encodeURIComponent(slug), state)]];
    token.level = level;
    children.unshift(token);

    token = new state.Token('link_close', 'a', -1);
    token.level = level;
    children.push(token);
  }
});
md.use(toc, {
  markerPattern: /^\[TOC\]/im,
  includeLevel: [1, 2, 3],
  slugify: s => String(s).trim().toLowerCase().replace(/\s+/g, '-'),
});

export default async (markdown) => {
  mermaid_blocks = [];
  let html = md.render(markdown);
  const blocks = mermaid_blocks;

  for (let index = 0; index < blocks.length; ++index) {
    const definition = blocks[index];
    const svg_id = `mermaid-${md5(definition).slice(0, 8)}-${index}`;
    const [light, dark] = await Promise.all([
      render_mermaid(definition, `${svg_id}-light`, 'default'),
      render_mermaid(definition, `${svg_id}-dark`, 'dark'),
    ]);
    html = html.replace(
      `<pre class="mermaid-placeholder">${index}</pre>`,
      `<figure class="mermaid"><div class="mermaid-light">${light}</div><div class="mermaid-dark">${dark}</div></figure>`);
  }
  return html;
}
