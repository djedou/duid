
use std::collections::HashMap;


pub fn ring_shadow_styles() -> HashMap<String, String> {
    let mut ring_shadow = HashMap::new();

    let _ = ring_shadow.insert("ring-shadow".to_owned(), "box-shadow: 0 0 0 var(--duid-ring-offset-width) var(--duid-ring-offset-color), var(--duid-ring-shadow);".to_owned());
    let _ = ring_shadow.insert("ring-shadow-var".to_owned(), "--duid-ring-shadow".to_owned());
    
    ring_shadow
}
