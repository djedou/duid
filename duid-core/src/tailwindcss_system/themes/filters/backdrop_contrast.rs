use std::collections::HashMap;


pub(crate) fn backdrop_contrast() -> HashMap<String, String> {
    let mut backdrop_contrast = HashMap::new();
    let _ = backdrop_contrast.insert("backdrop-contrast-0".to_owned(), "backdrop-filter: contrast(0);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-50".to_owned(), "backdrop-filter: contrast(.5);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-75".to_owned(), "backdrop-filter: contrast(.75);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-100".to_owned(), "backdrop-filter: contrast(1);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-125".to_owned(), "backdrop-filter: contrast(1.25);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-150".to_owned(), "backdrop-filter: contrast(1.5);".to_owned());
    let _ = backdrop_contrast.insert("backdrop-contrast-200".to_owned(), "backdrop-filter: contrast(2);".to_owned());

    backdrop_contrast
}