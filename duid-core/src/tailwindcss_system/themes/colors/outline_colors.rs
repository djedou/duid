use std::collections::HashMap;

pub fn outline_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("outline-color".to_owned(), "outline-color".to_owned());
    
    styles
}
