const fs = require("fs");
const md5 = require("md5");
const hljs = require('highlight.js');
const kt = require('katex');
const tm = require('markdown-it-texmath').use(kt);
const mi = require('markdown-it-linkify-images');
const markdown_it = require('markdown-it');

const save_component = function(vue_content, comp_name) {
    fs.writeFileSync(`compiled/${comp_name}.vue`, vue_content);
}

const md = markdown_it({
    linkify: true,
    typography: true,
    highlight: function (str, lang) {
        const just_vue_code = (lang === 'vue');
        const just_vue_component = (lang === 'VUE');
        lang = lang.toLowerCase();
        const is_vue = (lang === 'vue');

        if (lang && hljs.getLanguage(lang)) {
            return `<pre class="hljs"><code>${hljs.highlight(lang, str).value}</code></pre>`;
        } else {
            const highlight_result = `<pre class="hljs"><code>${hljs.highlightAuto(str).value}</code></pre>`;

            if (is_vue && !just_vue_code) {
                const comp_name = `comp-${md5(str)}`;
                save_component(str, comp_name);
                const component = `<div class="vue_in_posts_container"><${comp_name} class="vue_in_posts"/></div>`;

                if (just_vue_component) {
                    return `<pre hidden><code></code></pre>` + component;
                } else {
                    return highlight_result + component;
                }
            } else {
                return highlight_result;
            }
        }
    }
});
md.use(tm);
md.use(mi);

module.exports = (markdown) => {
    return md.render(markdown);
}
