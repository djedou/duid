use std::collections::HashMap;


pub fn ring_colors_styles() -> HashMap<String, String> {
    let mut ring_color = HashMap::new();
    let _ = ring_color.insert("ring-color-var".to_owned(), "--duid-ring-color".to_owned());
    
    ring_color
}
