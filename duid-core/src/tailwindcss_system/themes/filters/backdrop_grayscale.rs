use std::collections::HashMap;


pub(crate) fn backdrop_grayscale() -> HashMap<String, String> {
    let mut backdrop_grayscale = HashMap::new();
    let _ = backdrop_grayscale.insert("backdrop-grayscale-0".to_owned(), "backdrop-filter: grayscale(0);".to_owned());
    let _ = backdrop_grayscale.insert("backdrop-grayscale".to_owned(), "backdrop-filter: grayscale(100%);".to_owned());

    backdrop_grayscale
}