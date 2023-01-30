use std::collections::HashMap;


pub(crate) fn invert() -> HashMap<String, String> {
    let mut invert = HashMap::new();
    let _ = invert.insert("invert-0".to_owned(), "filter: invert(0);".to_owned());
    let _ = invert.insert("invert".to_owned(), "filter: invert(100%);".to_owned());

    invert
}