use std::collections::HashMap;


pub(crate) fn flex_grow() -> HashMap<String, String> {
    let mut flex_grow = HashMap::new();
    let _ = flex_grow.insert("grow".to_owned(), "flex-grow: 1;".to_owned());
    let _ = flex_grow.insert("grow-0".to_owned(), "flex-grow: 0;".to_owned());

    flex_grow
}