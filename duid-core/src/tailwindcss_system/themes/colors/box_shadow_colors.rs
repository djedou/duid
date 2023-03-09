use std::collections::HashMap;

pub fn box_shadow_colors_styles() -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let _ = styles.insert("shadow-color-var".to_owned(), "--duid-shadow-color".to_owned());
    let _ = styles.insert("inset-shadow-color-var".to_owned(), "--duid-inset-shadow-color".to_owned());
    
    styles
}
