use crate::article::Article;
use crate::config::Config;
use chrono::{NaiveDate, Utc};

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Parse date string in YYYY.MM.DD format to RFC 2822 format
fn parse_date_to_rfc2822(date_str: &str) -> String {
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    
    // Try to parse YYYY.MM.DD format
    let date_str = date_str.replace('.', "-");
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => {
            date.format("%a, %d %b %Y 00:00:00 GMT").to_string()
        }
        Err(_) => now,
    }
}

/// Generate RSS feed from articles
pub fn generate_rss(articles: &[Article], config: &Config) -> String {
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    
    let items: String = articles
        .iter()
        .take(20)
        .map(|article| {
            let pub_date = parse_date_to_rfc2822(&article.date);
            let url_path = article.url_path.as_deref().unwrap_or("");
            let link = format!("{}{}", config.site_url, url_path);
            let guid = &link;
            let author = &article.author;
            
            let category_tags: String = article
                .tags
                .iter()
                .map(|t| format!("      <category>{}</category>", escape_xml(t)))
                .collect::<Vec<_>>()
                .join("\n");
            
            format!(
                r#"    <item>
      <title>{}</title>
      <link>{}</link>
      <guid isPermaLink="true">{}</guid>
      <pubDate>{}</pubDate>
      <dc:creator>{}</dc:creator>
{}
    </item>"#,
                escape_xml(&article.title),
                escape_xml(&link),
                escape_xml(guid),
                pub_date,
                escape_xml(author),
                category_tags
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:dc="http://purl.org/dc/elements/1.1/">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <language>{}</language>
    <lastBuildDate>{}</lastBuildDate>
    <atom:link href="{}/rss.xml" rel="self" type="application/rss+xml" />
{}
  </channel>
</rss>"#,
        escape_xml(&config.site_name),
        escape_xml(&config.site_url),
        escape_xml(&config.site_description),
        &config.site_language,
        now,
        escape_xml(&config.site_url),
        items
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
        assert_eq!(escape_xml("&"), "&amp;");
    }

    #[test]
    fn test_parse_date_to_rfc2822() {
        let result = parse_date_to_rfc2822("2024.03.23");
        assert!(result.contains("23 Mar 2024"));
    }
}
