use std::collections::HashMap;


pub(crate) fn outline_width() -> HashMap<String, String> {
    let mut outline_width = HashMap::new();
    let _ = outline_width.insert("outline-0".to_owned(), "outline-width: 0px;".to_owned());
    let _ = outline_width.insert("outline-1".to_owned(), "outline-width: 1px;".to_owned());
    let _ = outline_width.insert("outline-2".to_owned(), "outline-width: 2px;".to_owned());
    let _ = outline_width.insert("outline-4".to_owned(), "outline-width: 4px;".to_owned());
    let _ = outline_width.insert("outline-8".to_owned(), "outline-width: 8px;".to_owned());

    outline_width
}