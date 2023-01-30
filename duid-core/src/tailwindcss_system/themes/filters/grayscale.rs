use std::collections::HashMap;


pub(crate) fn grayscale() -> HashMap<String, String> {
    let mut grayscale = HashMap::new();
    let _ = grayscale.insert("grayscale-0".to_owned(), "filter: grayscale(0);".to_owned());
    let _ = grayscale.insert("grayscale".to_owned(), "filter: grayscale(100%);".to_owned());

    grayscale
}