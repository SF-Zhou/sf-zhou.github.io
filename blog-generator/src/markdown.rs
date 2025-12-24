use katex;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use regex::Regex;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub struct MarkdownRenderer {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        
        MarkdownRenderer {
            syntax_set,
            theme_set,
        }
    }

    /// Map language aliases to syntect language names
    fn map_language<'a>(&self, lang: &'a str) -> &'a str {
        match lang.to_lowercase().as_str() {
            "" => "txt",
            "c++" | "cpp" => "C++",
            "yml" => "YAML",
            "asm" | "assembly" | "nasm" => "Assembly x86 (NASM)",
            "bash" | "shell" | "sh" => "Bash",
            "cmake" => "CMake",
            "json" => "JSON",
            "lua" => "Lua",
            "protobuf" | "proto" => "Protocol Buffers",
            "python" | "py" => "Python",
            "rust" | "rs" => "Rust",
            "toml" => "TOML",
            "yaml" => "YAML",
            "c" => "C",
            "css" => "CSS",
            "html" => "HTML",
            "java" => "Java",
            "javascript" | "js" => "JavaScript",
            "typescript" | "ts" => "TypeScript",
            "go" => "Go",
            "markdown" | "md" => "Markdown",
            "sql" => "SQL",
            "xml" => "XML",
            "makefile" | "make" => "Makefile",
            "diff" => "Diff",
            _ => lang,
        }
    }

    /// Highlight code using syntect
    fn highlight_code(&self, code: &str, lang: &str) -> String {
        let mapped_lang = self.map_language(lang);
        let syntax = self.syntax_set
            .find_syntax_by_name(mapped_lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        
        match highlighted_html_for_string(code, &self.syntax_set, syntax, theme) {
            Ok(highlighted) => highlighted,
            Err(_) => {
                // Fallback to escaped HTML
                let escaped = html_escape(code);
                format!("<pre class=\"language-{}\"><code class=\"language-{}\">{}</code></pre>", 
                    lang.to_lowercase(), lang.to_lowercase(), escaped)
            }
        }
    }

    /// Process math expressions, but skip content inside code blocks
    fn process_math(&self, content: &str) -> String {
        // First, protect code blocks from math processing
        let code_block_placeholder = "___CODE_BLOCK_PLACEHOLDER___";
        let inline_code_placeholder = "___INLINE_CODE_PLACEHOLDER___";
        
        // Protect fenced code blocks (```...```)
        let fenced_code_re = Regex::new(r"```[\s\S]*?```").unwrap();
        let fenced_codes: Vec<String> = fenced_code_re
            .find_iter(content)
            .map(|m| m.as_str().to_string())
            .collect();
        
        let mut result = content.to_string();
        for (i, _) in fenced_codes.iter().enumerate() {
            result = fenced_code_re.replace(&result, &format!("{}{}___", code_block_placeholder, i)).to_string();
        }
        
        // Protect inline code (`...`)
        let inline_code_re = Regex::new(r"`[^`]+`").unwrap();
        let inline_codes: Vec<String> = inline_code_re
            .find_iter(&result)
            .map(|m| m.as_str().to_string())
            .collect();
        
        for (i, _) in inline_codes.iter().enumerate() {
            result = inline_code_re.replace(&result, &format!("{}{}___", inline_code_placeholder, i)).to_string();
        }
        
        // Use a placeholder for display math to avoid being matched by inline math
        let display_placeholder = "___DISPLAY_MATH_PLACEHOLDER___";
        
        // Process display math ($$...$$) - must be done before inline math
        let display_math_re = Regex::new(r"\$\$([^$]+)\$\$").unwrap();
        let display_maths: Vec<String> = display_math_re
            .captures_iter(&result)
            .map(|caps| caps[1].to_string())
            .collect();
        
        let mut display_rendered: Vec<String> = Vec::new();
        for math in &display_maths {
            let rendered = match katex::render_with_opts(math, katex::Opts::builder().display_mode(true).build().unwrap()) {
                Ok(r) => format!("<eqn><p class=\"katex-display\">{}</p></eqn>", r),
                Err(_) => format!("<eqn><p class=\"katex-display\">{}</p></eqn>", html_escape(math)),
            };
            display_rendered.push(rendered);
        }
        
        // Replace display math with placeholders
        let mut counter = 0;
        result = display_math_re.replace_all(&result, |_caps: &regex::Captures| {
            let placeholder = format!("{}{}___", display_placeholder, counter);
            counter += 1;
            placeholder
        }).to_string();
        
        // Process inline math ($...$) - now safe because $$ are replaced
        let inline_math_re = Regex::new(r"\$([^$\n]+)\$").unwrap();
        result = inline_math_re.replace_all(&result, |caps: &regex::Captures| {
            let math = &caps[1];
            match katex::render_with_opts(math, katex::Opts::builder().display_mode(false).build().unwrap()) {
                Ok(rendered) => rendered,
                Err(_) => html_escape(math),
            }
        }).to_string();
        
        // Restore display math from placeholders
        for (i, rendered) in display_rendered.into_iter().enumerate() {
            let placeholder = format!("{}{}___", display_placeholder, i);
            result = result.replace(&placeholder, &rendered);
        }
        
        // Restore inline code
        for (i, code) in inline_codes.into_iter().enumerate() {
            let placeholder = format!("{}{}___", inline_code_placeholder, i);
            result = result.replace(&placeholder, &code);
        }
        
        // Restore fenced code blocks
        for (i, code) in fenced_codes.into_iter().enumerate() {
            let placeholder = format!("{}{}___", code_block_placeholder, i);
            result = result.replace(&placeholder, &code);
        }
        
        result
    }

    /// Generate slug for anchor links
    fn slugify(&self, s: &str) -> String {
        s.trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Render markdown to HTML
    pub fn render(&self, markdown: &str) -> String {
        // First, process math expressions in the markdown
        let processed_markdown = self.process_math(markdown);
        
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        
        let parser = Parser::new_ext(&processed_markdown, options);
        
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_buffer = String::new();
        let mut toc_entries: Vec<(u32, String, String)> = Vec::new();
        let should_insert_toc = processed_markdown.contains("[TOC]");
        
        let events: Vec<Event> = parser.collect();
        let mut processed_events = Vec::new();
        
        for event in events {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_block_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    code_buffer.clear();
                }
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                    let highlighted = self.highlight_code(&code_buffer, &code_block_lang);
                    processed_events.push(Event::Html(CowStr::from(highlighted)));
                }
                Event::Text(text) if in_code_block => {
                    code_buffer.push_str(&text);
                }
                Event::Start(Tag::Heading(_level, _, _)) => {
                    // We'll handle headings specially to add anchors
                    processed_events.push(event);
                }
                Event::End(Tag::Heading(_level, _, _)) => {
                    processed_events.push(event);
                }
                Event::Text(text) => {
                    // Check for [TOC] marker
                    if text.trim() == "[TOC]" && should_insert_toc {
                        // We'll generate TOC after collecting all headings
                        processed_events.push(Event::Html(CowStr::from("<!-- TOC_PLACEHOLDER -->")));
                    } else {
                        processed_events.push(Event::Text(text));
                    }
                }
                Event::Start(Tag::Image(_link_type, url, title)) => {
                    // Handle images with figcaption
                    let url_str = url.to_string();
                    let title_str = title.to_string();
                    processed_events.push(Event::Html(CowStr::from(format!(
                        "<figure><a href=\"{}\"><img src=\"{}\" alt=\"{}\" title=\"{}\" tabindex=\"-1\">",
                        url_str, url_str, title_str, title_str
                    ))));
                }
                Event::End(Tag::Image(_, _, title)) => {
                    let title_str = title.to_string();
                    if !title_str.is_empty() {
                        processed_events.push(Event::Html(CowStr::from(format!(
                            "</a><figcaption>{}</figcaption></figure>",
                            title_str
                        ))));
                    } else {
                        processed_events.push(Event::Html(CowStr::from("</a></figure>")));
                    }
                }
                _ => {
                    processed_events.push(event);
                }
            }
        }
        
        // Convert events to HTML
        let mut html_output = String::new();
        html::push_html(&mut html_output, processed_events.into_iter());
        
        // Post-process: Add anchor links to headings
        let heading_re = Regex::new(r"<h([1-6])>([^<]+)</h([1-6])>").unwrap();
        html_output = heading_re.replace_all(&html_output, |caps: &regex::Captures| {
            let level: u32 = caps[1].parse().unwrap();
            let text = &caps[2];
            let slug = self.slugify(text);
            let encoded_slug = percent_encoding::utf8_percent_encode(&slug, percent_encoding::NON_ALPHANUMERIC).to_string();
            
            // Collect TOC entries
            toc_entries.push((level, slug.clone(), text.to_string()));
            
            format!("<h{0}><a href=\"#{1}\">{2}</a></h{0}>", level, encoded_slug, text)
        }).to_string();
        
        // Generate and insert TOC if needed
        if should_insert_toc && !toc_entries.is_empty() {
            let toc_html = self.generate_toc(&toc_entries);
            html_output = html_output.replace("<!-- TOC_PLACEHOLDER -->", &toc_html);
        }
        
        html_output
    }

    /// Generate table of contents HTML
    fn generate_toc(&self, entries: &[(u32, String, String)]) -> String {
        if entries.is_empty() {
            return String::new();
        }
        
        let mut toc = String::from("<div class=\"table-of-contents\"><ul>");
        let mut current_level = 1;
        
        for (level, slug, text) in entries {
            let level = *level;
            if level > 3 {
                continue; // Only include h1, h2, h3
            }
            
            while current_level < level {
                toc.push_str("<ul>");
                current_level += 1;
            }
            while current_level > level {
                toc.push_str("</ul>");
                current_level -= 1;
            }
            
            let encoded_slug = percent_encoding::utf8_percent_encode(slug, percent_encoding::NON_ALPHANUMERIC).to_string();
            toc.push_str(&format!("<li><a href=\"#{}\">{}</a></li>", encoded_slug, text));
        }
        
        while current_level > 1 {
            toc.push_str("</ul>");
            current_level -= 1;
        }
        
        toc.push_str("</ul></div>");
        toc
    }
}

/// HTML escape utility
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn test_slugify() {
        let renderer = MarkdownRenderer::new();
        assert_eq!(renderer.slugify("Hello World"), "hello-world");
        assert_eq!(renderer.slugify("Test 123"), "test-123");
    }
}
