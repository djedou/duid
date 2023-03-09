use std::collections::HashMap;


pub fn background_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("bg-color".to_owned(), "background-color".to_owned());
    
    styles
}
