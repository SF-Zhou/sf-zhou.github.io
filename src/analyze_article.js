function analyze_article(markdown, article_filename, default_author) {
  let date, tags_string, author;
  const match = article_filename.match(/^\[([^\]]+)\]/);
  if (match) {
    // Format: [date tags author...]
    // `tags` is one comma-separated token; everything after it is the author,
    // which may contain spaces (e.g. a display name instead of a GitHub handle).
    const info = match[1].trim().split(/\s+/);
    date = info[0];
    tags_string = info[1];
    author = info.slice(2).join(' ');
    article_filename = article_filename.replace(/^\[([^\]]+)\]/, '');
  }

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
