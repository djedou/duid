use std::collections::HashMap;


pub(crate) fn saturate() -> HashMap<String, String> {
    let mut saturate = HashMap::new();
    let _ = saturate.insert("saturate-0".to_owned(), "filter: saturate(0);".to_owned());
    let _ = saturate.insert("saturate-50".to_owned(), "filter: saturate(.5);".to_owned());
    let _ = saturate.insert("saturate-100".to_owned(), "filter: saturate(1);".to_owned());
    let _ = saturate.insert("saturate-150".to_owned(), "filter: saturate(1.5);".to_owned());
    let _ = saturate.insert("saturate-200".to_owned(), "filter: saturate(2);".to_owned());

    saturate
}