use std::collections::HashMap;


pub(crate) fn list_style_position() -> HashMap<String, String> {
    let mut list_style_position = HashMap::new();
    let _ = list_style_position.insert("list-inside".to_owned(), "list-style-position: inside;".to_owned());
    let _ = list_style_position.insert("list-outside".to_owned(), "list-style-position: outside;".to_owned());

    list_style_position
}