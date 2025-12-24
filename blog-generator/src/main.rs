mod article;
mod config;
mod list_articles;
mod markdown;
mod rss;
mod template;

use article::{analyze_article, Article};
use clap::Parser;
use config::Config;
use list_articles::list_articles;
use markdown::MarkdownRenderer;
use mustache::Data;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use template::{render_template_string, str_data, vec_data};

#[derive(Parser, Debug)]
#[command(author, version, about = "Static blog generator that converts Markdown to HTML")]
struct Args {
    /// Path to the configuration file (TOML format)
    #[arg(short, long)]
    config: PathBuf,

    /// Path to the posts directory (overrides config)
    #[arg(short, long)]
    posts: Option<PathBuf>,

    /// Path to the output directory (overrides config)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to the templates directory
    #[arg(short, long, default_value = "src")]
    templates: PathBuf,
}

/// Write file only if content has changed
fn write_when_change<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        let old_content = fs::read_to_string(path)?;
        if old_content == content {
            return Ok(());
        }
    }
    fs::write(path, content)
}

/// Create article data for template rendering
fn create_article_data(article: &Article) -> HashMap<String, Data> {
    let mut data = HashMap::new();
    data.insert("title".to_string(), str_data(&article.title));
    data.insert("date".to_string(), str_data(&article.date));
    data.insert("author".to_string(), str_data(&article.author));
    data.insert("filename".to_string(), str_data(&article.filename));
    
    if let Some(ref html) = article.html {
        data.insert("article".to_string(), str_data(html));
    }
    
    if let Some(ref url_path) = article.url_path {
        data.insert("url_path".to_string(), str_data(url_path));
    }
    
    // Tags as array with named key
    let tags_data: Vec<HashMap<String, Data>> = article
        .tags
        .iter()
        .map(|tag| {
            let mut tag_map = HashMap::new();
            tag_map.insert("name".to_string(), str_data(tag));
            tag_map
        })
        .collect();
    data.insert("tags".to_string(), vec_data(tags_data));
    
    data
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Load configuration
    let mut config = Config::from_file(&args.config)?;
    
    // Override paths if provided
    if let Some(ref posts) = args.posts {
        config.posts_path = posts.to_string_lossy().to_string();
    }
    if let Some(ref output) = args.output {
        config.output_path = output.to_string_lossy().to_string();
    }
    
    // List all articles
    let (dirs, articles_path) = list_articles(&config.posts_path, &config.article_format);
    
    // Create output directories
    fs::create_dir_all(&config.output_path)?;
    for dir in &dirs {
        fs::create_dir_all(Path::new(&config.output_path).join(dir))?;
    }
    
    // Load templates
    let article_template_path = args.templates.join("article.html");
    let card_template_path = args.templates.join("card.html");
    let profile_template_path = args.templates.join("profile.md");
    
    let article_template = fs::read_to_string(&article_template_path)?;
    let card_template = fs::read_to_string(&card_template_path)?;
    
    // Initialize markdown renderer
    let renderer = MarkdownRenderer::new();
    
    let mut articles_info: Vec<Article> = Vec::new();
    
    // Process each article
    for article_path in &articles_path {
        let article_dir = Path::new(article_path).parent().unwrap_or(Path::new(""));
        let article_filename = Path::new(article_path)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let full_path = Path::new(&config.posts_path).join(article_path);
        let article_content = fs::read_to_string(&full_path)?;
        
        // Analyze article metadata
        let mut article = analyze_article(&article_content, &article_filename, &config.default_author);
        
        // Render markdown to HTML
        article.html = Some(renderer.render(&article.markdown));
        
        let hidden = article.tags.contains(&"Hidden".to_string());
        
        // Generate comment script if not hidden
        let comment = if hidden {
            String::new()
        } else if let (Some(ref repo), Some(ref repo_id)) = (&config.github_repo, &config.github_repo_id) {
            format!(
                r#"<script src="https://giscus.app/client.js"
      data-repo="{}"
      data-repo-id="{}"
      data-category="Announcements"
      data-category-id="DIC_kwDOBNDkVs4CA6GQ"
      data-mapping="title"
      data-reactions-enabled="0"
      data-emit-metadata="0"
      data-input-position="bottom"
      data-theme="preferred_color_scheme"
      data-lang="en"
      crossorigin="anonymous"
      async>
    </script>"#,
                repo, repo_id
            )
        } else {
            String::new()
        };
        
        // Prepare template data
        let mut view = HashMap::new();
        view.insert(
            "title_string".to_string(),
            str_data(&format!("{} | {}", article.title, config.site_name)),
        );
        view.insert("title".to_string(), str_data(&article.title));
        view.insert("date".to_string(), str_data(&article.date));
        view.insert("author".to_string(), str_data(&article.author));
        view.insert(
            "tags".to_string(),
            str_data(&serde_json::to_string(&article.tags).unwrap_or_default()),
        );
        if let Some(ref html) = article.html {
            view.insert("article".to_string(), str_data(html));
        }
        view.insert("comment".to_string(), str_data(&comment));
        view.insert("web_master".to_string(), str_data(&config.web_master));
        if let Some(ref ga_id) = config.google_analytics_id {
            view.insert("google_analytics_id".to_string(), str_data(ga_id));
        }
        
        // Render article HTML
        let render_result = render_template_string(&article_template, view)?;
        
        // Clean up article for info storage
        article.html = None;
        article.markdown = String::new();
        
        let html_filename = format!("{}{}", article.filename, if hidden { ".htm" } else { ".html" });
        article.url_path = Some(format!("/{}/{}", article_dir.to_string_lossy(), html_filename));
        
        if !hidden {
            articles_info.push(article.clone());
        }
        
        // Write HTML file
        let html_path = Path::new(&config.output_path)
            .join(article_dir)
            .join(&html_filename);
        write_when_change(&html_path, &render_result)?;
        
        println!("Generated: {}", html_path.display());
    }
    
    // Sort articles by date (newest first)
    articles_info.sort_by(|a, b| {
        match b.date.cmp(&a.date) {
            std::cmp::Ordering::Equal => a.title.cmp(&b.title),
            other => other,
        }
    });
    
    // Generate index page
    let articles_data: Vec<HashMap<String, Data>> = articles_info
        .iter()
        .map(create_article_data)
        .collect();
    
    let mut card_view = HashMap::new();
    card_view.insert("articles".to_string(), vec_data(articles_data));
    let index_result = render_template_string(&card_template, card_view)?;
    
    let mut index_view = HashMap::new();
    index_view.insert("title_string".to_string(), str_data(&config.site_name));
    index_view.insert("title".to_string(), str_data(&config.site_name));
    index_view.insert("article".to_string(), str_data(&index_result));
    index_view.insert("web_master".to_string(), str_data(&config.web_master));
    if let Some(ref ga_id) = config.google_analytics_id {
        index_view.insert("google_analytics_id".to_string(), str_data(ga_id));
    }
    
    let index_html = render_template_string(&article_template, index_view)?;
    let index_path = Path::new(&config.output_path).join("index.html");
    write_when_change(&index_path, &index_html)?;
    println!("Generated: {}", index_path.display());
    
    // Generate index.json
    let json_path = Path::new(&config.output_path).join("index.json");
    let json_content = serde_json::to_string_pretty(&articles_info)?;
    write_when_change(&json_path, &json_content)?;
    println!("Generated: {}", json_path.display());
    
    // Generate RSS feed
    let rss_content = rss::generate_rss(&articles_info, &config);
    let rss_path = Path::new(&config.output_path).join("rss.xml");
    write_when_change(&rss_path, &rss_content)?;
    println!("Generated: {}", rss_path.display());
    
    // Generate profile README
    if let Some(ref profile_path) = config.profile_path {
        if profile_template_path.exists() {
            let profile_template = fs::read_to_string(&profile_template_path)?;
            
            let top_articles: Vec<HashMap<String, Data>> = articles_info
                .iter()
                .take(5)
                .map(create_article_data)
                .collect();
            
            let mut profile_view = HashMap::new();
            profile_view.insert("articles".to_string(), vec_data(top_articles));
            
            let profile_content = render_template_string(&profile_template, profile_view)?;
            
            fs::create_dir_all(profile_path)?;
            let profile_readme_path = Path::new(profile_path).join("README.md");
            write_when_change(&profile_readme_path, &profile_content)?;
            println!("Generated: {}", profile_readme_path.display());
        }
    }
    
    println!("\nBuild completed successfully!");
    Ok(())
}

