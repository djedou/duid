
use std::collections::HashMap;


pub fn ring_offset_colors_styles() -> HashMap<String, String> {
    let mut ring_offset_color = HashMap::new();
    let _ = ring_offset_color.insert("ring-offset-color-var".to_owned(), "--duid-ring-offset-color".to_owned());
    
    ring_offset_color
}
