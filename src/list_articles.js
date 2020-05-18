const fs = require('fs').promises;
const lstat = require('fs').lstatSync;
const path = require('path');

async function list_dir(base_path, dir_path) {
  return (await fs.readdir(path.join(base_path, dir_path)))
    .map(name => path.join(dir_path, name));
}

async function find_dir(base_path, dir_path) {
  const items = await list_dir(base_path, dir_path);
  const dirs = items.filter(
    item => lstat(path.join(base_path, item)).isDirectory());
  return (await Promise.all(dirs.map(dir => find_dir(base_path, dir))))
    .reduce((a, b) => a.concat(b), dirs);
}

async function list_articles(base_path, article_format) {
  const dirs = await find_dir(base_path, '');
  const ext_name = `.${article_format}`;
  const articles_path =
    (await Promise.all(dirs.map(async dir => {
      const items = await list_dir(base_path, dir);
      const files = items.filter(
        item => lstat(path.join(base_path, item)).isFile());
      return files.filter(filename => filename.endsWith(ext_name));
    }))).reduce((a, b) => a.concat(b), []);
  return {
    dirs, articles_path
  }
}

module.exports = list_articles
