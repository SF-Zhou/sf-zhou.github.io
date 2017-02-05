const katex = require("katex");
const marked = require("marked");
const highlight = require("highlight.js");

module.exports = (markdown) => {
    const renderer = new marked.Renderer();
    renderer.em = text => `_${text}_`;

    const old_paragraph = renderer.paragraph;
    renderer.paragraph = function (text) {
        if (text.startsWith('$$') && text.endsWith('$$')) {
            const formula = katex.renderToString(text.substring(2, text.length - 2), { displayMode: true });
            return '<div class="math">' + formula + '</div>'
        } else {
            return old_paragraph(text);
        }
    };

    marked.setOptions({
        renderer: renderer,
        highlight: function (code) {
            return highlight.highlightAuto(code).value;
        }
    });

    return marked(markdown);
}
