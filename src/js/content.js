const content = window.content;

const title = Buffer.from(content.title, 'base64').toString();
const date = Buffer.from(content.date, 'base64').toString();
const author = Buffer.from(content.author, 'base64').toString();
const article = Buffer.from(content.article, 'base64').toString();
const tags = ["测试", "Markdown", "中文之美", "Excited!"];

export default {
    title, date, author, article, tags
};
