use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Article {
    pub title: String,
    pub markdown: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
}

/// Analyzes markdown content and extracts metadata from the filename
/// 
/// Filename format: [date tags author]article_name.md
/// Example: [2024.03.23 Rust,C++,CMake]link_rust_in_cmake_cpp_env.md
pub fn analyze_article(markdown: &str, article_filename: &str, default_author: &str) -> Article {
    let mut filename = article_filename.to_string();
    let mut date = String::new();
    let mut tags = Vec::new();
    let mut author = default_author.to_string();

    // Parse metadata from filename: [date tags author]filename
    let re = Regex::new(r"^\[([^\]]+)\]").unwrap();
    if let Some(captures) = re.captures(&filename) {
        let info_str = &captures[1];
        let parts: Vec<&str> = info_str.split_whitespace().collect();
        
        if !parts.is_empty() {
            date = parts[0].to_string();
        }
        if parts.len() > 1 {
            tags = parts[1].split(',').map(|s| s.to_string()).collect();
        }
        if parts.len() > 2 {
            author = parts[2].to_string();
        }
        
        // Remove the metadata prefix from filename
        filename = re.replace(&filename, "").to_string();
    }

    // Extract title from markdown (first H1 heading) or use filename
    let mut markdown_content = markdown.to_string();
    let title = if markdown.starts_with("# ") {
        let lines: Vec<&str> = markdown.lines().collect();
        let title = lines[0][2..].to_string();
        markdown_content = lines[1..].join("\n");
        title
    } else {
        filename.clone()
    };

    Article {
        title,
        markdown: markdown_content,
        date,
        author,
        tags,
        filename,
        html: None,
        url_path: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_article_with_metadata() {
        let markdown = "# Test Title\n\nSome content";
        let filename = "[2024.03.23 Rust,C++,CMake]test_file";
        let article = analyze_article(markdown, filename, "default_author");
        
        assert_eq!(article.title, "Test Title");
        assert_eq!(article.date, "2024.03.23");
        assert_eq!(article.tags, vec!["Rust", "C++", "CMake"]);
        assert_eq!(article.author, "default_author");
        assert_eq!(article.filename, "test_file");
    }

    #[test]
    fn test_analyze_article_with_author() {
        let markdown = "# Test Title\n\nSome content";
        let filename = "[2024.03.23 Rust CustomAuthor]test_file";
        let article = analyze_article(markdown, filename, "default_author");
        
        assert_eq!(article.author, "CustomAuthor");
    }

    #[test]
    fn test_analyze_article_without_h1() {
        let markdown = "Some content without title";
        let filename = "[2024.03.23 Rust]test_file";
        let article = analyze_article(markdown, filename, "default_author");
        
        assert_eq!(article.title, "test_file");
        assert_eq!(article.markdown, "Some content without title");
    }
}
