use std::collections::HashMap;


pub(crate) fn align_items() -> HashMap<String, String> {
    let mut align_items = HashMap::new();

    let _ = align_items.insert("items-start".to_owned(), "align-items: flex-start;".to_owned());
    let _ = align_items.insert("items-end".to_owned(), "align-items: flex-end;".to_owned());
    let _ = align_items.insert("items-center".to_owned(), "align-items: center;".to_owned());
    let _ = align_items.insert("items-baseline".to_owned(), "align-items: baseline;".to_owned());
    let _ = align_items.insert("items-stretch".to_owned(), "align-items: stretch;".to_owned());

    align_items
}