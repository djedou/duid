use std::collections::HashMap;


pub(crate) fn object_position() -> HashMap<String, String> {
    let mut object_position = HashMap::new();
    let _ = object_position.insert("object-bottom".to_owned(), "object-position: bottom;".to_owned());
    let _ = object_position.insert("object-center".to_owned(), "object-position: center;".to_owned());
    let _ = object_position.insert("object-left".to_owned(), "object-position: left;".to_owned());
    let _ = object_position.insert("object-left-bottom".to_owned(), "object-position: left bottom;".to_owned());
    let _ = object_position.insert("object-left-top".to_owned(), "object-position: left top;".to_owned());
    let _ = object_position.insert("object-right".to_owned(), "object-position: right;".to_owned());
    let _ = object_position.insert("object-right-bottom".to_owned(), "object-position: right bottom;".to_owned());
    let _ = object_position.insert("object-right-top".to_owned(), "object-position: right top;".to_owned());
    let _ = object_position.insert("object-top".to_owned(), "object-position: top;".to_owned());

    object_position
}