use std::collections::HashMap;


pub(crate) fn backdrop_invert() -> HashMap<String, String> {
    let mut backdrop_invert = HashMap::new();
    let _ = backdrop_invert.insert("backdrop-invert-0".to_owned(), "backdrop-filter: invert(0);".to_owned());
    let _ = backdrop_invert.insert("backdrop-invert".to_owned(), "backdrop-filter: invert(100%);".to_owned());

    backdrop_invert
}