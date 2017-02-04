function analyze_article(markdown, article_filename) {
    let title = article_filename;
    if (markdown.startsWith('# ')) {
        const lines = markdown.split('\n');
        title = lines.shift().substr(2);
        markdown = lines.join('\n');
    }

    return {
        title, markdown
    }
}

module.exports = analyze_article;
