function analyze_article(markdown, article_filename, default_author) {
  let info = [];
  const match = article_filename.match(/^\[([^\]]+)\]/);
  if (match) {
    info = match[1].split(' ');
    article_filename = article_filename.replace(/^\[([^\]]+)\]/, '');
  }
  let [date, tags_string, author] = info;

  let tags = tags_string ? tags_string.split(',') : [];
  author = author || default_author;

  let title = article_filename;
  if (markdown.startsWith('# ')) {
    const lines = markdown.split('\n');
    title = lines.shift().substr(2);
    markdown = lines.join('\n');
  }

  return {
    title, markdown, date, author, tags, filename: article_filename
  }
}

export default analyze_article;
