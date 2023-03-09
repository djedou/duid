use std::collections::HashMap;

pub fn accent_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("accent-color".to_owned(), "accent-color".to_owned());
    
    styles
}
