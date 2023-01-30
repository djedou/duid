use std::collections::HashMap;


pub(crate) fn clear() -> HashMap<String, String> {
    let mut clear = HashMap::new();
    let _ = clear.insert("clear-left".to_owned(), "clear: left;".to_owned());
    let _ = clear.insert("clear-right".to_owned(), "clear: right;".to_owned());
    let _ = clear.insert("clear-both".to_owned(), "clear: both;".to_owned());
    let _ = clear.insert("clear-none".to_owned(), "clear: none;".to_owned());

    clear
}