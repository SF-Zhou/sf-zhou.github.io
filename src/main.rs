mod article;
mod config;
mod list_articles;
mod markdown;
mod rss;

use article::{analyze_article, Article};
use clap::Parser;
use config::Config;
use indoc::formatdoc;
use list_articles::list_articles;
use markdown::MarkdownRenderer;
use mustache::Data;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "Static blog generator")]
struct Args {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(short, long)]
    posts: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long, default_value = "src")]
    templates: PathBuf,
}

const CARD_TEMPLATE: &str = r#"{{#articles}}
<div class="card">
  <div class="abstract">
    <a href="{{{url_path}}}" class="article_title">
      <p>{{{title}}}</p>
    </a>
    <div class="article_info">
      <div class="article_base">
        <p class="date">{{{date}}}</p>
        <a href="https://github.com/{{{author}}}">
          <p class="author"> {{{author}}} </p>
        </a>
      </div>
      <div class="tags">
        {{#tags}}
        <p class="tag">
          <a href="/#/{{{name}}}">{{{name}}}</a>
        </p>
        {{/tags}}
      </div>
    </div>
  </div>
</div>
{{/articles}}"#;

fn write_if_changed<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    if path.exists() && fs::read_to_string(path).ok().as_deref() == Some(content) {
        return Ok(());
    }
    fs::write(path, content)
}

fn str_data(s: &str) -> Data {
    Data::String(s.to_string())
}

fn vec_data(items: Vec<HashMap<String, Data>>) -> Data {
    Data::Vec(items.into_iter().map(Data::Map).collect())
}

fn render_template(template: &str, data: HashMap<String, Data>) -> Result<String, Box<dyn std::error::Error>> {
    let compiled = mustache::compile_str(template)?;
    let mut output = Vec::new();
    compiled.render_data(&mut output, &Data::Map(data))?;
    Ok(String::from_utf8(output)?)
}

fn create_article_view(article: &Article) -> HashMap<String, Data> {
    let mut view = HashMap::new();
    view.insert("title".to_string(), str_data(&article.title));
    view.insert("date".to_string(), str_data(&article.date));
    view.insert("author".to_string(), str_data(&article.author));
    view.insert("filename".to_string(), str_data(&article.filename));
    
    if let Some(ref html) = article.html {
        view.insert("article".to_string(), str_data(html));
    }
    if let Some(ref url) = article.url_path {
        view.insert("url_path".to_string(), str_data(url));
    }
    
    let tags: Vec<HashMap<String, Data>> = article
        .tags
        .iter()
        .map(|tag| {
            let mut m = HashMap::new();
            m.insert("name".to_string(), str_data(tag));
            m
        })
        .collect();
    view.insert("tags".to_string(), vec_data(tags));
    
    view
}

fn generate_giscus_script(config: &Config) -> String {
    match (&config.github_repo, &config.github_repo_id, &config.giscus_category, &config.giscus_category_id) {
        (Some(repo), Some(repo_id), Some(category), Some(category_id)) => {
            formatdoc! {r#"
                <script src="https://giscus.app/client.js"
                  data-repo="{repo}"
                  data-repo-id="{repo_id}"
                  data-category="{category}"
                  data-category-id="{category_id}"
                  data-mapping="title"
                  data-reactions-enabled="0"
                  data-emit-metadata="0"
                  data-input-position="bottom"
                  data-theme="preferred_color_scheme"
                  data-lang="en"
                  crossorigin="anonymous"
                  async>
                </script>"#}
        }
        _ => String::new(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = Config::from_file(&args.config)?;
    
    if let Some(ref posts) = args.posts {
        config.posts_path = posts.to_string_lossy().to_string();
    }
    if let Some(ref output) = args.output {
        config.output_path = output.to_string_lossy().to_string();
    }
    
    let (dirs, articles_path) = list_articles(&config.posts_path, &config.article_format);
    
    fs::create_dir_all(&config.output_path)?;
    for dir in &dirs {
        fs::create_dir_all(Path::new(&config.output_path).join(dir))?;
    }
    
    let article_template = fs::read_to_string(args.templates.join("article.html"))?;
    let profile_template_path = args.templates.join("profile.md");
    
    let renderer = MarkdownRenderer::new();
    let giscus_script = generate_giscus_script(&config);
    
    let mut articles_info: Vec<Article> = Vec::new();
    
    for article_path in &articles_path {
        let dir = Path::new(article_path).parent().unwrap_or(Path::new(""));
        let filename = Path::new(article_path)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let content = fs::read_to_string(Path::new(&config.posts_path).join(article_path))?;
        let mut article = analyze_article(&content, &filename, &config.default_author);
        
        article.html = Some(renderer.render(&article.markdown));
        
        let hidden = article.tags.contains(&"Hidden".to_string());
        let comment = if hidden { String::new() } else { giscus_script.clone() };
        
        let mut view = HashMap::new();
        view.insert("title_string".to_string(), str_data(&format!("{} | {}", article.title, config.site_name)));
        view.insert("title".to_string(), str_data(&article.title));
        view.insert("date".to_string(), str_data(&article.date));
        view.insert("author".to_string(), str_data(&article.author));
        view.insert("tags".to_string(), str_data(&serde_json::to_string(&article.tags)?));
        if let Some(ref html) = article.html {
            view.insert("article".to_string(), str_data(html));
        }
        view.insert("comment".to_string(), str_data(&comment));
        view.insert("web_master".to_string(), str_data(&config.web_master));
        if let Some(ref ga_id) = config.google_analytics_id {
            view.insert("google_analytics_id".to_string(), str_data(ga_id));
        }
        
        let html_content = render_template(&article_template, view)?;
        
        article.html = None;
        article.markdown = String::new();
        
        let ext = if hidden { ".htm" } else { ".html" };
        let html_filename = format!("{}{}", article.filename, ext);
        article.url_path = Some(format!("/{}/{}", dir.to_string_lossy(), html_filename));
        
        if !hidden {
            articles_info.push(article.clone());
        }
        
        let html_path = Path::new(&config.output_path).join(dir).join(&html_filename);
        write_if_changed(&html_path, &html_content)?;
        println!("Generated: {}", html_path.display());
    }
    
    articles_info.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));
    
    // Generate index page
    let articles_view: Vec<HashMap<String, Data>> = articles_info.iter().map(create_article_view).collect();
    let mut card_data = HashMap::new();
    card_data.insert("articles".to_string(), vec_data(articles_view));
    let index_content = render_template(CARD_TEMPLATE, card_data)?;
    
    let mut index_view = HashMap::new();
    index_view.insert("title_string".to_string(), str_data(&config.site_name));
    index_view.insert("title".to_string(), str_data(&config.site_name));
    index_view.insert("article".to_string(), str_data(&index_content));
    index_view.insert("web_master".to_string(), str_data(&config.web_master));
    if let Some(ref ga_id) = config.google_analytics_id {
        index_view.insert("google_analytics_id".to_string(), str_data(ga_id));
    }
    
    let index_html = render_template(&article_template, index_view)?;
    write_if_changed(Path::new(&config.output_path).join("index.html"), &index_html)?;
    println!("Generated: {}/index.html", config.output_path);
    
    // Generate index.json
    let json_content = serde_json::to_string_pretty(&articles_info)?;
    write_if_changed(Path::new(&config.output_path).join("index.json"), &json_content)?;
    println!("Generated: {}/index.json", config.output_path);
    
    // Generate RSS
    let rss_content = rss::generate_rss(&articles_info, &config);
    write_if_changed(Path::new(&config.output_path).join("rss.xml"), &rss_content)?;
    println!("Generated: {}/rss.xml", config.output_path);
    
    // Generate profile README
    if let Some(ref profile_path) = config.profile_path {
        if profile_template_path.exists() {
            let profile_template = fs::read_to_string(&profile_template_path)?;
            let top_articles: Vec<HashMap<String, Data>> = articles_info.iter().take(5).map(create_article_view).collect();
            let mut profile_data = HashMap::new();
            profile_data.insert("articles".to_string(), vec_data(top_articles));
            let profile_content = render_template(&profile_template, profile_data)?;
            
            fs::create_dir_all(profile_path)?;
            write_if_changed(Path::new(profile_path).join("README.md"), &profile_content)?;
            println!("Generated: {}/README.md", profile_path);
        }
    }
    
    println!("\nBuild completed!");
    Ok(())
}
