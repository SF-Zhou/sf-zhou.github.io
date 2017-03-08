const fs = require("mz/fs");
const path = require("path");
const mustache = require("mustache");

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

    // delete exists vue component in posts
    await Promise.all((await fs.readdir('compiled')).filter(filename => filename.endsWith(".vue")).map(async filename => {
        await fs.unlink(`compiled/${filename}`);
    }));

    const article_template_name = "./templates/article.tpl";
    const article_template = fs.readFileSync(article_template_name).toString();

    let articles_info = [];
    await Promise.all(articles_path.map(async article_path => {
        const article_dir = path.dirname(article_path);
        const article_filename = path.basename(article_path).replace(/\.[^.]+$/, '');
        const article_content = (await fs.readFile(path.join(config.posts_path, article_path))).toString();

        const article = analyze_article(article_content, article_filename);
        article.html = marked(article.markdown);

        const view = {
            index: "undefined",
            title_string: article.title,
            title: JSON.stringify(article.title),
            date: article.date ? JSON.stringify(article.date) : "undefined",
            author: JSON.stringify(article.author),
            tags: JSON.stringify(article.tags),
            article: article.html
        };
        const render_result = mustache.render(article_template, view);

        delete article.html;
        delete article.markdown;
        const html_filename = article.filename + '.html';
        article.url_path = path.join(article_dir, html_filename);

        if (article.date) {
            articles_info.push(article);
        }

        const html_path = path.join(config.output_path, article.url_path);
        await fs.writeFile(html_path, render_result);
    }));

    // sort articles by date
    articles_info.sort((a, b) => {
        if (a.date < b.date) return 1;
        if (a.date > b.date) return -1;
        return a.title <= b.title;
    });

    const article={};
    const view = {
        title_string: config.site_name,
        index: JSON.stringify(articles_info),
        title: JSON.stringify(config.site_name),
        date: "undefined",
        author: "undefined",
        tags: "undefined",
        article: "undefined"
    };
    const render_result = mustache.render(article_template, view);
    const html_path = path.join(config.output_path, "index.html");
    await fs.writeFile(html_path, render_result);

    const vue_in_posts = (await fs.readdir('compiled')).filter(filename => filename.endsWith(".vue"));
    const componenet_command = vue_in_posts.map(filename => {
        return `Vue.component('${path.basename(filename, '.vue')}', require('./${filename}'));`
    }).join('\n');
    const plugin_template = `import Vue from 'vue'\nexports.install = function() { ${componenet_command} };`
    await fs.writeFile('compiled/vue_in_posts.js', plugin_template);
}

main();
