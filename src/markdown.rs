use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use regex::Regex;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static CODE_BLOCK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"```[\s\S]*?```").unwrap());
static DISPLAY_MATH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\$([\s\S]*?)\$\$").unwrap());
static HEADING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<h([1-6])>([^<]+)</h([1-6])>").unwrap());

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
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    fn map_language(&self, lang: &str) -> &'static str {
        match lang.to_lowercase().as_str() {
            "" | "txt" | "text" => "Plain Text",
            "c++" | "cpp" => "C++",
            "c" => "C",
            "rust" | "rs" => "Rust",
            "python" | "py" => "Python",
            "javascript" | "js" => "JavaScript",
            "typescript" | "ts" => "TypeScript",
            "go" => "Go",
            "java" => "Java",
            "bash" | "shell" | "sh" => "Bash",
            "json" => "JSON",
            "yaml" | "yml" => "YAML",
            "toml" => "TOML",
            "html" => "HTML",
            "css" => "CSS",
            "sql" => "SQL",
            "xml" => "XML",
            "markdown" | "md" => "Markdown",
            "makefile" | "make" => "Makefile",
            "cmake" => "CMake",
            "diff" => "Diff",
            "protobuf" | "proto" => "Protocol Buffers",
            "lua" => "Lua",
            "asm" | "assembly" | "nasm" => "Assembly x86 (NASM)",
            _ => "Plain Text",
        }
    }

    fn highlight_code(&self, code: &str, lang: &str) -> String {
        let syntax_name = self.map_language(lang);
        let syntax = self
            .syntax_set
            .find_syntax_by_name(syntax_name)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let theme = &self.theme_set.themes["base16-ocean.dark"];

        highlighted_html_for_string(code, &self.syntax_set, syntax, theme)
            .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", escape_html(code)))
    }

    fn render_math(&self, math: &str, display: bool) -> String {
        let opts = katex::Opts::builder().display_mode(display).build().unwrap();
        match katex::render_with_opts(math, opts) {
            Ok(rendered) if display => format!("<eqn><p class=\"katex-display\">{}</p></eqn>", rendered),
            Ok(rendered) => rendered,
            Err(_) => escape_html(math),
        }
    }

    /// Preprocess markdown to render display math ($$...$$) before markdown parsing
    fn preprocess_display_math(&self, content: &str) -> String {
        // First, protect code blocks from math processing
        let mut result = content.to_string();
        let code_blocks: Vec<(usize, usize, String)> = CODE_BLOCK_RE
            .find_iter(content)
            .map(|m| (m.start(), m.end(), m.as_str().to_string()))
            .collect();
        
        // Replace code blocks with placeholders
        for (i, (start, end, _)) in code_blocks.iter().enumerate().rev() {
            let placeholder = format!("___CODE_BLOCK_{}___", i);
            result.replace_range(*start..*end, &placeholder);
        }
        
        // Process display math ($$...$$)
        result = DISPLAY_MATH_RE.replace_all(&result, |caps: &regex::Captures| {
            let math = &caps[1];
            self.render_math(math.trim(), true)
        }).to_string();
        
        // Restore code blocks
        for (i, (_, _, original)) in code_blocks.iter().enumerate() {
            let placeholder = format!("___CODE_BLOCK_{}___", i);
            result = result.replace(&placeholder, original);
        }
        
        result
    }

    fn process_inline_math(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check for inline math ($...$) - single dollar signs only
            if chars[i] == '$' {
                // Make sure it's not a double $$ (those are already processed)
                let is_double = i + 1 < chars.len() && chars[i + 1] == '$';
                if !is_double {
                    if let Some(end) = self.find_closing_single_dollar(&chars, i + 1) {
                        let math: String = chars[i + 1..end].iter().collect();
                        if !math.is_empty() && !math.contains('\n') {
                            result.push_str(&self.render_math(&math, false));
                            i = end + 1;
                            continue;
                        }
                    }
                }
            }
            result.push(chars[i]);
            i += 1;
        }
        result
    }

    fn find_closing_single_dollar(&self, chars: &[char], start: usize) -> Option<usize> {
        for (i, &c) in chars[start..].iter().enumerate() {
            if c == '$' {
                return Some(start + i);
            }
            if c == '\n' {
                return None;
            }
        }
        None
    }

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

    pub fn render(&self, markdown: &str) -> String {
        // First, preprocess display math
        let preprocessed = self.preprocess_display_math(markdown);
        
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(&preprocessed, options);
        let mut events: Vec<Event> = Vec::new();
        let mut in_code_block = false;
        let mut code_lang = String::new();
        let mut code_buffer = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    code_buffer.clear();
                }
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                    let highlighted = self.highlight_code(&code_buffer, &code_lang);
                    events.push(Event::Html(CowStr::from(highlighted)));
                }
                Event::Text(text) if in_code_block => {
                    code_buffer.push_str(&text);
                }
                Event::Code(code) => {
                    // Inline code - don't process math
                    events.push(Event::Html(CowStr::from(format!("<code>{}</code>", escape_html(&code)))));
                }
                Event::Text(text) => {
                    // Process inline math in regular text
                    let processed = self.process_inline_math(&text);
                    events.push(Event::Html(CowStr::from(processed)));
                }
                Event::Start(Tag::Image(_link_type, url, title)) => {
                    events.push(Event::Html(CowStr::from(format!(
                        "<figure><a href=\"{}\"><img src=\"{}\" alt=\"{}\" title=\"{}\" tabindex=\"-1\">",
                        url, url, title, title
                    ))));
                }
                Event::End(Tag::Image(_, _, title)) => {
                    if title.is_empty() {
                        events.push(Event::Html(CowStr::from("</a></figure>")));
                    } else {
                        events.push(Event::Html(CowStr::from(format!("</a><figcaption>{}</figcaption></figure>", title))));
                    }
                }
                _ => events.push(event),
            }
        }

        let mut html_output = String::new();
        html::push_html(&mut html_output, events.into_iter());

        // Add anchor links to headings
        html_output = HEADING_RE
            .replace_all(&html_output, |caps: &regex::Captures| {
                let level = &caps[1];
                let text = &caps[2];
                let slug = self.slugify(text);
                let encoded = percent_encoding::utf8_percent_encode(&slug, percent_encoding::NON_ALPHANUMERIC);
                format!("<h{0}><a href=\"#{1}\">{2}</a></h{0}>", level, encoded, text)
            })
            .to_string();

        html_output
    }
}

fn escape_html(s: &str) -> String {
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
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn test_slugify() {
        let renderer = MarkdownRenderer::new();
        assert_eq!(renderer.slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_display_math() {
        let renderer = MarkdownRenderer::new();
        let result = renderer.preprocess_display_math("test $$ x^2 $$ end");
        assert!(result.contains("katex"));
    }
}
