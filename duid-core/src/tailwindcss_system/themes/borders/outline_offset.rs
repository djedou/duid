use std::collections::HashMap;


pub(crate) fn outline_offset() -> HashMap<String, String> {
    let mut outline_offset = HashMap::new();
    let _ = outline_offset.insert("outline-offset-0".to_owned(), "outline-offset: 0px;".to_owned());
    let _ = outline_offset.insert("outline-offset-1".to_owned(), "outline-offset: 1px;".to_owned());
    let _ = outline_offset.insert("outline-offset-2".to_owned(), "outline-offset: 2px;".to_owned());
    let _ = outline_offset.insert("outline-offset-4".to_owned(), "outline-offset: 4px;".to_owned());
    let _ = outline_offset.insert("outline-offset-8".to_owned(), "outline-offset: 8px;".to_owned());

    outline_offset
}