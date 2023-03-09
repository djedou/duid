use std::collections::HashMap;

pub fn divide_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("divide-color".to_owned(), "border-color".to_owned());
        
    styles
}
