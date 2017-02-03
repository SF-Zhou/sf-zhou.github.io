const fs = require("fs");
const path = require("path");
const marked = require("marked");

const filename = "./posts/class/博客系统 Markdown 绘制功能性测试文档.md";
const filepath = path.join(__dirname, filename);
let title = path.basename(filepath).slice(0, -3);

let content = fs.readFileSync(filepath).toString();
if (content.startsWith('# ')) {
    const lines = content.split('\n');
    title = lines.shift().substr(2);
    content = lines.join('\n');
}

const author_info = "<a href='https://github.com/sf-zhou'>SF-Zhou</a>";
const marked_content = marked(content);

const article_template_name = "./templates/article.tpl";
const article_template_path = path.join(__dirname, article_template_name);
const article_template = fs.readFileSync(article_template_path).toString();

let result = article_template
    .replace('{{ title_string }}', title)
    .replace('{{ title }}', new Buffer(title).toString('base64'))
    .replace('{{ date }}', new Buffer("2017.02.02").toString('base64'))
    .replace('{{ author }}', new Buffer(author_info).toString('base64'))
    .replace('{{ article }}', new Buffer(marked_content).toString('base64'));
result = result.replace('{{ title }}', title);

fs.writeFileSync("index.html", result);
fs.writeFileSync("public/index.html", result);
