use std::collections::HashMap;


pub(crate) fn background_size() -> HashMap<String, String> {
    let mut background_size = HashMap::new();
    let _ = background_size.insert("bg-auto".to_owned(), "background-size: auto;".to_owned());
    let _ = background_size.insert("bg-cover".to_owned(), "background-size: cover;".to_owned());
    let _ = background_size.insert("bg-contain".to_owned(), "background-size: contain;".to_owned());

    background_size
}