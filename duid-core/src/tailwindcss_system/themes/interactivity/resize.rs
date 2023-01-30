use std::collections::HashMap;


pub(crate) fn resize() -> HashMap<String, String> {
    let mut resize = HashMap::new();
    let _ = resize.insert("resize-none".to_owned(), "resize: none;".to_owned());
    let _ = resize.insert("resize-y".to_owned(), "resize: vertical;".to_owned());
    let _ = resize.insert("resize-x".to_owned(), "resize: horizontal;".to_owned());
    let _ = resize.insert("resize".to_owned(), "resize: both;".to_owned());

    resize
}