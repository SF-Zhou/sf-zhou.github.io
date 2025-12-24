use mustache::{self, Data};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Render a template with the given data
pub fn render_template<P: AsRef<Path>>(
    template_path: P,
    data: HashMap<String, Data>,
) -> Result<String, Box<dyn std::error::Error>> {
    let template_content = fs::read_to_string(template_path)?;
    render_template_string(&template_content, data)
}

/// Render a template string with the given data
pub fn render_template_string(
    template: &str,
    data: HashMap<String, Data>,
) -> Result<String, Box<dyn std::error::Error>> {
    let compiled = mustache::compile_str(template)?;
    let mut output = Vec::new();
    compiled.render_data(&mut output, &Data::Map(data))?;
    Ok(String::from_utf8(output)?)
}

/// Helper to create Data::String
pub fn str_data(s: &str) -> Data {
    Data::String(s.to_string())
}

/// Helper to create Data::Bool
#[allow(dead_code)]
pub fn bool_data(b: bool) -> Data {
    Data::Bool(b)
}

/// Helper to create Data::Vec from a vector of HashMaps
pub fn vec_data(items: Vec<HashMap<String, Data>>) -> Data {
    Data::Vec(items.into_iter().map(Data::Map).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template_string() {
        let template = "Hello, {{{ name }}}!";
        let mut data = HashMap::new();
        data.insert("name".to_string(), str_data("World"));
        
        let result = render_template_string(template, data).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_render_template_with_array() {
        let template = "{{#items}}{{{ value }}}{{/items}}";
        let mut data = HashMap::new();
        
        let mut item1 = HashMap::new();
        item1.insert("value".to_string(), str_data("A"));
        let mut item2 = HashMap::new();
        item2.insert("value".to_string(), str_data("B"));
        
        data.insert("items".to_string(), vec_data(vec![item1, item2]));
        
        let result = render_template_string(template, data).unwrap();
        assert_eq!(result, "AB");
    }
}
