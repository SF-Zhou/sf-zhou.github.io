const marked = require("marked");
const highlight = require("highlight.js");

module.exports = (markdown) => {
    marked.setOptions({
        highlight: function (code) {
            return highlight.highlightAuto(code).value;
        }
    });

    return marked(markdown);
}
