use std::collections::HashMap;


pub(crate) fn justify_content() -> HashMap<String, String> {
    let mut justify_content = HashMap::new();
    let _ = justify_content.insert("justify-start".to_owned(), "justify-content: flex-start;".to_owned());
    let _ = justify_content.insert("justify-end".to_owned(), "justify-content: flex-end;".to_owned());
    let _ = justify_content.insert("justify-center".to_owned(), "justify-content: center;".to_owned());
    let _ = justify_content.insert("justify-between".to_owned(), "justify-content: space-between;".to_owned());
    let _ = justify_content.insert("justify-around".to_owned(), "justify-content: space-around;".to_owned());
    let _ = justify_content.insert("justify-evenly".to_owned(), "justify-content: space-evenly;".to_owned());

    justify_content
}