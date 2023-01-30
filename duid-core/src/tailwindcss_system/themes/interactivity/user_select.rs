use std::collections::HashMap;


pub(crate) fn user_select() -> HashMap<String, String> {
    let mut user_select = HashMap::new();
    let _ = user_select.insert("select-none".to_owned(), "user-select: none;".to_owned());
    let _ = user_select.insert("select-text".to_owned(), "user-select: text;".to_owned());
    let _ = user_select.insert("select-all".to_owned(), "user-select: all;".to_owned());
    let _ = user_select.insert("select-auto".to_owned(), "user-select: auto;".to_owned());

    user_select
}