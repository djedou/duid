use std::collections::HashMap;


pub(crate) fn box_sizing() -> HashMap<String, String> {
    let mut box_sizing = HashMap::new();
    let _ = box_sizing.insert("box-border".to_owned(), "box-sizing: border-box;".to_owned());
    let _ = box_sizing.insert("box-content".to_owned(), "box-sizing: content-box;".to_owned());
    
    box_sizing
}
