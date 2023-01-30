use std::collections::HashMap;


pub(crate) fn sepia() -> HashMap<String, String> {
    let mut sepia = HashMap::new();
    let _ = sepia.insert("sepia-0".to_owned(), "filter: sepia(0);".to_owned());
    let _ = sepia.insert("sepia".to_owned(), "filter: sepia(100%);".to_owned());

    sepia
}
