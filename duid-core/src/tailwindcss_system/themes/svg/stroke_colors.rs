use std::collections::HashMap;

pub fn stroke_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("stroke-color".to_owned(), "stroke".to_owned());
        
    styles
}
