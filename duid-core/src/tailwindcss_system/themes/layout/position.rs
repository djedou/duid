use std::collections::HashMap;


pub(crate) fn position() -> HashMap<String, String> {
    let mut position = HashMap::new();
    let _ = position.insert("static".to_owned(), "position: static;".to_owned());
    let _ = position.insert("fixed".to_owned(), "position: fixed;".to_owned());
    let _ = position.insert("absolute".to_owned(), "position: absolute;".to_owned());
    let _ = position.insert("relative".to_owned(), "position: relative;".to_owned());
    let _ = position.insert("sticky".to_owned(), "position: sticky;".to_owned());
    

    position
}