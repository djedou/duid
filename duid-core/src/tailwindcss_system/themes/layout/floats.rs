use std::collections::HashMap;


pub(crate) fn floats() -> HashMap<String, String> {
    let mut floats = HashMap::new();
    let _ = floats.insert("float-right".to_owned(), "float: right;".to_owned());
    let _ = floats.insert("float-left".to_owned(), "float: left;".to_owned());
    let _ = floats.insert("float-none".to_owned(), "float: none;".to_owned());
    

    floats
}
