use std::collections::HashMap;


pub(crate) fn overflow() -> HashMap<String, String> {
    let mut overflow = HashMap::new();
    let _ = overflow.insert("overflow-auto".to_owned(), "overflow: auto;".to_owned());
    let _ = overflow.insert("overflow-hidden".to_owned(), "overflow: hidden;".to_owned());
    let _ = overflow.insert("overflow-clip".to_owned(), "overflow: clip;".to_owned());
    let _ = overflow.insert("overflow-visible".to_owned(), "overflow: visible;".to_owned());
    let _ = overflow.insert("overflow-scroll".to_owned(), "overflow: scroll;".to_owned());
    let _ = overflow.insert("overflow-x-auto".to_owned(), "overflow-x: auto;".to_owned());
    let _ = overflow.insert("overflow-y-auto".to_owned(), "overflow-y: auto;".to_owned());
    let _ = overflow.insert("overflow-x-hidden".to_owned(), "overflow-x: hidden;".to_owned());
    let _ = overflow.insert("overflow-y-hidden".to_owned(), "overflow-y: hidden;".to_owned());
    let _ = overflow.insert("overflow-x-clip".to_owned(), "overflow-x: clip;".to_owned());
    let _ = overflow.insert("overflow-y-clip".to_owned(), "overflow-y: clip;".to_owned());
    let _ = overflow.insert("overflow-x-visible".to_owned(), "overflow-x: visible;".to_owned());
    let _ = overflow.insert("overflow-y-visible".to_owned(), "overflow-y: visible;".to_owned());
    let _ = overflow.insert("overflow-x-scroll".to_owned(), "overflow-x: scroll;".to_owned());
    let _ = overflow.insert("overflow-y-scroll".to_owned(), "overflow-y: scroll;".to_owned());

    overflow
}