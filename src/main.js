import md5 from 'md5';
import { promises as fs } from 'fs';
import { existsSync as exists } from 'fs';
import { join, dirname, basename } from 'path';
import mustache from 'mustache';
import download from 'download';
import { fileTypeFromBuffer } from 'file-type';

import marked from './marked.js';
import list_articles from './list_articles.js';
import analyze_article from './analyze_article.js';

async function write_when_change(file_path, new_content) {
  if (exists(file_path)) {
    const old_content = (await fs.readFile(file_path)).toString();
    if (old_content === new_content) {
      return;
    }
  }
  await fs.writeFile(file_path, new_content);
}

async function main() {
  const config = JSON.parse(await fs.readFile('./config.json'));

  const { dirs, articles_path } =
    await list_articles(config.posts_path, config.article_format);

  await fs.mkdir(config.output_path, { recursive: true });
  for (const dir of [...dirs]) {
    await fs.mkdir(join(config.output_path, dir), { recursive: true });
  }

  const article_template_name = './src/article.html';
  const article_template = (await fs.readFile(article_template_name)).toString();

  let articles_info = [];
  await Promise.all(articles_path.map(async article_path => {
    const article_dir = dirname(article_path);
    const article_filename =
      basename(article_path).replace(/\.[^.]+$/, '');
    let article_content =
      (await fs.readFile(join(config.posts_path, article_path)))
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
        const type_info = await fileTypeFromBuffer(image_data);
        let ext_name = type_info.ext;
        if (ext_name === 'xml') {
          ext_name = 'svg';
        }
        const image_filename = `${md5(image_data)}.${ext_name}`;
        const image_path = `images/${image_filename}`;
        await fs.writeFile(
          join(config.posts_path, image_path), image_data);

        const replacing_path =
          join(relative_image_folder_path, image_path);
        replacing_list[url] = replacing_path;
      }));

      for (const key in replacing_list) {
        article_content = article_content.replace(key, replacing_list[key]);
      }
      await fs.writeFile(
        join(config.posts_path, article_path), article_content);
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
      web_master: config.web_master,
      google_analytics_id: config.google_analytics_id
    };
    const render_result = mustache.render(article_template, view);

    delete article.html;
    delete article.markdown;
    const hidden = article.tags.includes('Hidden');
    const html_filename = article.filename + (hidden ? '.htm' : '.html');
    article.url_path = join('/', article_dir, html_filename);

    if (!hidden) {
      articles_info.push(article);
    }

    const html_path = join(config.output_path, article.url_path);
    await write_when_change(html_path, render_result);
  }));

  // sort articles by date
  articles_info.sort((a, b) => {
    if (a.date < b.date) return 1;
    if (a.date > b.date) return -1;
    return a.title <= b.title;
  });

  const index_template_name = './src/card.html';
  const index_template = (await fs.readFile(index_template_name)).toString();
  const index_result =
    mustache.render(index_template, { articles: articles_info });

  const view = {
    title_string: config.site_name,
    title: config.site_name,
    article: index_result,
    web_master: config.web_master,
    google_analytics_id: config.google_analytics_id
  };
  const render_result = mustache.render(article_template, view);
  const html_path = join(config.output_path, 'index.html');
  await write_when_change(html_path, render_result);

  const json_path = join(config.output_path, 'index.json');
  await write_when_change(json_path, JSON.stringify(articles_info, null, 2));

  const profile_template_name = './src/profile.md';
  const profile_template = (await fs.readFile(profile_template_name)).toString();
  const profile = mustache.render(profile_template, { articles: articles_info.slice(0, 5) });
  const profile_path = join(config.profile_path, 'README.md');
  await fs.mkdir(config.profile_path, { recursive: true });
  await write_when_change(profile_path, profile);
}

main();
