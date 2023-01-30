use std::collections::HashMap;


pub(crate) fn text_overflow() -> HashMap<String, String> {
    let mut text_overflow = HashMap::new();
    let _ = text_overflow.insert("truncate".to_owned(), "overflow: hidden;text-overflow: ellipsis;white-space: nowrap;".to_owned());
    let _ = text_overflow.insert("text-ellipsis".to_owned(), "text-overflow: ellipsis;".to_owned());
    let _ = text_overflow.insert("text-clip".to_owned(), "text-overflow: clip;".to_owned());

    text_overflow
}