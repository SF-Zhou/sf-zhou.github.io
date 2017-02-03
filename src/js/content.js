const content = window.content;

export default {
    title: Buffer.from(content.title, 'base64').toString(),
    date: Buffer.from(content.date, 'base64').toString(),
    author: Buffer.from(content.author, 'base64').toString(),
    article: Buffer.from(content.article, 'base64').toString(),
    tags: ["测试", "Markdown", "中文之美", "Excited!"]
};
