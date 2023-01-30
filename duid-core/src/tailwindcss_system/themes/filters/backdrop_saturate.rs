use std::collections::HashMap;


pub(crate) fn backdrop_saturate() -> HashMap<String, String> {
    let mut backdrop_saturate = HashMap::new();
    let _ = backdrop_saturate.insert("backdrop-saturate-0".to_owned(), "backdrop-filter: saturate(0);".to_owned());
    let _ = backdrop_saturate.insert("backdrop-saturate-50".to_owned(), "backdrop-filter: saturate(.5);".to_owned());
    let _ = backdrop_saturate.insert("backdrop-saturate-100".to_owned(), "backdrop-filter: saturate(1);".to_owned());
    let _ = backdrop_saturate.insert("backdrop-saturate-150".to_owned(), "backdrop-filter: saturate(1.5);".to_owned());
    let _ = backdrop_saturate.insert("backdrop-saturate-200".to_owned(), "backdrop-filter: saturate(2);".to_owned());

    backdrop_saturate
}