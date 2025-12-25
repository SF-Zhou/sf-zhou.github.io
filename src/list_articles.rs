use std::path::Path;
use walkdir::WalkDir;

/// Lists all directories and articles in the given path
pub fn list_articles<P: AsRef<Path>>(base_path: P, article_format: &str) -> (Vec<String>, Vec<String>) {
    let base = base_path.as_ref();
    let ext_name = format!(".{}", article_format);
    
    let mut dirs = Vec::new();
    let mut articles_path = Vec::new();
    
    for entry in WalkDir::new(base)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(base).unwrap();
        let relative_str = relative_path.to_string_lossy().to_string();
        
        if entry.file_type().is_dir() {
            dirs.push(relative_str);
        } else if entry.file_type().is_file() && relative_str.ends_with(&ext_name) {
            articles_path.push(relative_str);
        }
    }
    
    (dirs, articles_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_list_articles() {
        let dir = tempdir().unwrap();
        let base = dir.path();
        
        // Create directory structure
        fs::create_dir_all(base.join("subdir")).unwrap();
        File::create(base.join("subdir/test.md")).unwrap()
            .write_all(b"# Test").unwrap();
        
        let (dirs, articles) = list_articles(base, "md");
        
        assert!(dirs.contains(&"subdir".to_string()));
        assert!(articles.contains(&"subdir/test.md".to_string()));
    }
}
