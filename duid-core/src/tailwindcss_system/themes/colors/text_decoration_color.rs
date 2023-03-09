use std::collections::HashMap;

pub fn text_decoration_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("decoration-color".to_owned(), "text-decoration-color".to_owned());
    
    styles
}
