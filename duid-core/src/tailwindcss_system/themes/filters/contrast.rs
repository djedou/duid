use std::collections::HashMap;


pub(crate) fn contrast() -> HashMap<String, String> {
    let mut contrast = HashMap::new();
    let _ = contrast.insert("contrast-0".to_owned(), "filter: contrast(0);".to_owned());
    let _ = contrast.insert("contrast-50".to_owned(), "filter: contrast(.5);".to_owned());
    let _ = contrast.insert("contrast-75".to_owned(), "filter: contrast(.75);".to_owned());
    let _ = contrast.insert("contrast-100".to_owned(), "filter: contrast(1);".to_owned());
    let _ = contrast.insert("contrast-125".to_owned(), "filter: contrast(1.25);".to_owned());
    let _ = contrast.insert("contrast-150".to_owned(), "filter: contrast(1.5);".to_owned());
    let _ = contrast.insert("contrast-200".to_owned(), "filter: contrast(2);".to_owned());

    contrast
}