use std::collections::HashMap;


pub(crate) fn visibility() -> HashMap<String, String> {
    let mut visibility = HashMap::new();
    let _ = visibility.insert("visible".to_owned(), "visibility: visible;".to_owned());
    let _ = visibility.insert("invisible".to_owned(), "visibility: hidden;".to_owned());
    let _ = visibility.insert("collapse".to_owned(), "visibility: collapse;".to_owned());
    

    visibility
}