use std::collections::HashMap;


pub(crate) fn text_transform() -> HashMap<String, String> {
    let mut text_transform = HashMap::new();
    let _ = text_transform.insert("uppercase".to_owned(), "text-transform: uppercase;".to_owned());
    let _ = text_transform.insert("lowercase".to_owned(), "text-transform: lowercase;".to_owned());
    let _ = text_transform.insert("capitalize".to_owned(), "text-transform: capitalize;".to_owned());
    let _ = text_transform.insert("normal-case".to_owned(), "text-transform: none;".to_owned());

    text_transform
}