
use std::collections::HashMap;


pub(crate) fn box_decoration_break() -> HashMap<String, String> {
    let mut box_decoration_break = HashMap::new();
    let _ = box_decoration_break.insert("box-decoration-clone".to_owned(), "box-decoration-break: clone;".to_owned());
    let _ = box_decoration_break.insert("box-decoration-slice".to_owned(), "box-decoration-break: slice;".to_owned());
    
    box_decoration_break
}