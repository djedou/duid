use std::collections::HashMap;


pub(crate) fn background_origin() -> HashMap<String, String> {
    let mut background_origin = HashMap::new();
    let _ = background_origin.insert("bg-origin-border".to_owned(), "background-origin: border-box;".to_owned());
    let _ = background_origin.insert("bg-origin-padding".to_owned(), "background-origin: padding-box;".to_owned());
    let _ = background_origin.insert("bg-origin-content".to_owned(), "background-origin: content-box;".to_owned());

    background_origin
}