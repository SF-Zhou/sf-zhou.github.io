const fs = require("fs");
const md5 = require("md5");
const hljs = require('highlight.js');
const mk = require('markdown-it-katex');
const mi = require('markdown-it-linkify-images');
const markdown_it = require('markdown-it');

const save_component = function(vue_content, comp_name) {
    fs.writeFileSync(`compiled/${comp_name}.vue`, vue_content);
}

const md = markdown_it({
    linkify: true,
    typography: true,
    highlight: function (str, lang) {
        const highlight_result = `<pre class="hljs"><code>${hljs.highlightAuto(str).value}</code></pre>`;

        if (lang === 'Vue') {
            const comp_name = `comp-${md5(str)}`;
            save_component(str, comp_name);
            return highlight_result + `<div class="vue_in_posts_container"><${comp_name} class="vue_in_posts"/></div>`;
        } else {
            return highlight_result;
        }
    }
});
md.use(mk);
md.use(mi);

module.exports = (markdown) => {
    return md.render(markdown);
}
