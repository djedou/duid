use std::collections::HashMap;


pub(crate) fn place_items() -> HashMap<String, String> {
    let mut place_items = HashMap::new();
    let _ = place_items.insert("place-items-start".to_owned(), "place-items: start;".to_owned());
    let _ = place_items.insert("place-items-end".to_owned(), "place-items: end;".to_owned());
    let _ = place_items.insert("place-items-center".to_owned(), "place-items: center;".to_owned());
    let _ = place_items.insert("place-items-baseline".to_owned(), "place-items: baseline;".to_owned());
    let _ = place_items.insert("place-items-stretch".to_owned(), "place-items: stretch;".to_owned());

    place_items
}