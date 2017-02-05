const fs = require("mz/fs");
const path = require("path");

const marked = require("./marked");
const mkdir = require("./mkdir_recursive")
const list_articles = require('./list_articles');
const analyze_article = require("./analyze_article");

const config = require("../config.json");

async function main() {
    const {dirs, articles_path} = await list_articles(config.posts_path, config.article_format);

    mkdir(config.output_path);
    for (const dir of [...dirs]) {
        await mkdir(path.join(config.output_path, dir));
    }

    const article_template_name = "./templates/article.tpl";
    const article_template = fs.readFileSync(article_template_name).toString();

    await Promise.all(articles_path.map(async article_path => {
        const article_dir = path.dirname(article_path);
        const article_filename = path.basename(article_path).replace(/\.[^.]+$/, '');
        const article_content = (await fs.readFile(path.join(config.posts_path, article_path))).toString();

        const article = analyze_article(article_content, article_filename);
        const marked_content = marked(article.markdown);

        let render_result = article_template
            .replace('{{ title_string }}', article.title)
            .replace('{{ title }}', JSON.stringify(article.title))
            .replace('{{ date }}', JSON.stringify(article.date))
            .replace('{{ author }}', JSON.stringify(article.author))
            .replace('{{ tags }}', JSON.stringify(article.tags))
            .replace('{{ article }}', marked_content);

        const html_filename = article.filename + '.html';
        const html_path = path.join(config.output_path, article_dir, html_filename);
        await fs.writeFile(html_path, render_result);
    }));
}

main();
