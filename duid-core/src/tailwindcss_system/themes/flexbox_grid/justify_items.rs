use std::collections::HashMap;


pub(crate) fn justify_items() -> HashMap<String, String> {
    let mut justify_items = HashMap::new();
    let _ = justify_items.insert("justify-items-start".to_owned(), "justify-items: start;".to_owned());
    let _ = justify_items.insert("justify-items-end".to_owned(), "justify-items: end;".to_owned());
    let _ = justify_items.insert("justify-items-center".to_owned(), "justify-items: center;".to_owned());
    let _ = justify_items.insert("justify-items-stretch".to_owned(), "justify-items: stretch;".to_owned());

    justify_items
}