use std::collections::HashMap;

pub fn fill_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("fill-color".to_owned(), "fill".to_owned());
        
    styles
}
