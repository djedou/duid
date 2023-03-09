use std::collections::HashMap;

pub fn caret_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("caret-color".to_owned(), "caret-color".to_owned());
    
    styles
}
