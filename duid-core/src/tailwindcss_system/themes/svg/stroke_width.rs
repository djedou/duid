use std::collections::HashMap;


pub(crate) fn stroke_width() -> HashMap<String, String> {
    let mut stroke_width = HashMap::new();
    let _ = stroke_width.insert("stroke-0".to_owned(), "stroke-width: 0;".to_owned());
    let _ = stroke_width.insert("stroke-1".to_owned(), "stroke-width: 1;".to_owned());
    let _ = stroke_width.insert("stroke-2".to_owned(), "stroke-width: 2;".to_owned());

    stroke_width
}