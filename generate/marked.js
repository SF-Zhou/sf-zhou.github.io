const hljs = require('highlight.js');
const mk = require('markdown-it-katex');
const md = require('markdown-it')({
    highlight: function (str) {
        return hljs.highlightAuto(str).value;
    }
});
md.use(mk);

module.exports = (markdown) => {
    return md.render(markdown);
}
