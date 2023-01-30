use std::collections::HashMap;


pub(crate) fn backdrop_sepia() -> HashMap<String, String> {
    let mut backdrop_sepia = HashMap::new();
    let _ = backdrop_sepia.insert("backdrop-sepia-0".to_owned(), "backdrop-filter: sepia(0);".to_owned());
    let _ = backdrop_sepia.insert("backdrop-sepia".to_owned(), "backdrop-filter: sepia(100%);".to_owned());

    backdrop_sepia
}