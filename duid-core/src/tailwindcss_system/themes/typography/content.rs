use std::collections::HashMap;


pub(crate) fn content() -> HashMap<String, String> {
    let mut content = HashMap::new();
    let _ = content.insert("content-none".to_owned(), "content: auto;".to_owned());
    let _ = content.insert("content-".to_owned(), "content: customValue;".to_owned());

    content
}
