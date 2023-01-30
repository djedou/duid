use std::collections::HashMap;


pub(crate) fn background_position() -> HashMap<String, String> {
    let mut background_position = HashMap::new();
    let _ = background_position.insert("bg-bottom".to_owned(), "background-position: bottom;".to_owned());
    let _ = background_position.insert("bg-center".to_owned(), "background-position: center;".to_owned());
    let _ = background_position.insert("bg-left".to_owned(), "background-position: left;".to_owned());
    let _ = background_position.insert("bg-left-bottom".to_owned(), "background-position: left bottom;".to_owned());
    let _ = background_position.insert("bg-left-top".to_owned(), "background-position: left top;".to_owned());
    let _ = background_position.insert("bg-right".to_owned(), "background-position: right;".to_owned());
    let _ = background_position.insert("bg-right-bottom".to_owned(), "background-position: right bottom;".to_owned());
    let _ = background_position.insert("bg-right-top".to_owned(), "background-position: right top;".to_owned());
    let _ = background_position.insert("bg-top".to_owned(), "background-position: top;".to_owned());

    background_position
}