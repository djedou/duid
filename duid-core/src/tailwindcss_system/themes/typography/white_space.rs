use std::collections::HashMap;


pub(crate) fn white_space() -> HashMap<String, String> {
    let mut white_space = HashMap::new();
    let _ = white_space.insert("whitespace-normal".to_owned(), "white-space: normal;".to_owned());
    let _ = white_space.insert("whitespace-nowrap".to_owned(), "white-space: nowrap;".to_owned());
    let _ = white_space.insert("whitespace-pre".to_owned(), "white-space: pre;".to_owned());
    let _ = white_space.insert("whitespace-pre-line".to_owned(), "white-space: pre-line;".to_owned());
    let _ = white_space.insert("whitespace-pre-wrap".to_owned(), "white-space: pre-wrap;".to_owned());

    white_space
}