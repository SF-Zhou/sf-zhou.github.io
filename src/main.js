const md5 = require('md5');
const fs = require('mz/fs');
const path = require('path');
const mustache = require('mustache');
const download = require('download');
const file_type = require('file-type');

const marked = require('./marked');
const mkdir = require('./mkdir_recursive')
const list_articles = require('./list_articles');
const analyze_article = require('./analyze_article');

const config = require('../config.json');

const write_when_change =
    async function(file_path, new_content) {
  if (await fs.exists(file_path)) {
    const old_content = (await fs.readFile(file_path)).toString();
    if (old_content === new_content) {
      return;
    }
  }
  await fs.writeFile(file_path, new_content);
}

async function
main() {
  const {dirs, articles_path} =
      await list_articles(config.posts_path, config.article_format);

  mkdir(config.output_path);
  for (const dir of [...dirs]) {
    await mkdir(path.join(config.output_path, dir));
  }

  const article_template_name = './src/article.html';
  const article_template = fs.readFileSync(article_template_name).toString();

  let articles_info = [];
  await Promise.all(articles_path.map(async article_path => {
    const article_dir = path.dirname(article_path);
    const article_filename =
        path.basename(article_path).replace(/\.[^.]+$/, '');
    let article_content =
        (await fs.readFile(path.join(config.posts_path, article_path)))
            .toString();

    const outer_image_block_regexp = /\!\[.*\]\((http[s]?[^)]+)\)/g;
    const image_blocks = article_content.match(outer_image_block_regexp);
    if (image_blocks) {
      const relative_image_folder_path =
          '../'.repeat(article_path.match('/').length);

      const outer_image_url_regexp = /\(([^)]+)\)$/;
      const image_url_list =
          image_blocks.map(block => outer_image_url_regexp.exec(block)[1]);

      const replacing_list = {};
      await Promise.all(image_url_list.map(async url => {
        console.log('downloading...', url);
        const image_data = await download(url);
        let ext_name = file_type(image_data).ext;
        if (ext_name === 'xml') {
          ext_name = 'svg';
        }
        const image_filename = `${md5(image_data)}.${ext_name}`;
        const image_path = `images/${image_filename}`;
        await fs.writeFile(
            path.join(config.posts_path, image_path), image_data);

        const replacing_path =
            path.join(relative_image_folder_path, image_path);
        replacing_list[url] = replacing_path;
      }));

      for (const key in replacing_list) {
        article_content = article_content.replace(key, replacing_list[key]);
      }
      await fs.writeFile(
          path.join(config.posts_path, article_path), article_content);
    }

    const article = analyze_article(article_content, article_filename);
    article.html = marked(article.markdown);

    const view = {
      title_string: `${article.title} | ${config.site_name}`,
      title: article.title,
      date: article.date,
      author: article.author,
      tags: JSON.stringify(article.tags),
      article: article.html,
      year: (new Date()).getFullYear(),
      web_master: config.web_master
    };
    const render_result = mustache.render(article_template, view);

    delete article.html;
    delete article.markdown;
    const html_filename = article.filename + '.html';
    article.url_path = path.join('/', article_dir, html_filename);

    if (article.date) {
      articles_info.push(article);
    }

    const html_path = path.join(config.output_path, article.url_path);
    await write_when_change(html_path, render_result);
  }));

  // sort articles by date
  articles_info.sort((a, b) => {
    if (a.date < b.date) return 1;
    if (a.date > b.date) return -1;
    return a.title <= b.title;
  });

  const index_template_name = './src/card.html';
  const index_template = fs.readFileSync(index_template_name).toString();
  const index_result =
      mustache.render(index_template, {articles: articles_info});

  const view = {
    title_string: config.site_name,
    title: config.site_name,
    article: index_result,
    year: (new Date()).getFullYear(),
    web_master: config.web_master
  };
  const render_result = mustache.render(article_template, view);
  const html_path = path.join(config.output_path, 'index.html');
  await write_when_change(html_path, render_result);

  const json_path = path.join(config.output_path, 'index.json');
  await write_when_change(json_path, JSON.stringify(articles_info, null, 2));
}

main();
