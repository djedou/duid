use std::collections::HashMap;


pub(crate) fn object_fit() -> HashMap<String, String> {
    let mut object_fit = HashMap::new();
    let _ = object_fit.insert("object-contain".to_owned(), "object-fit: contain;".to_owned());
    let _ = object_fit.insert("object-cover".to_owned(), "object-fit: cover;".to_owned());
    let _ = object_fit.insert("object-fill".to_owned(), "object-fit: fill;".to_owned());
    let _ = object_fit.insert("object-none".to_owned(), "object-fit: none;".to_owned());
    let _ = object_fit.insert("object-scale-down".to_owned(), "object-fit: scale-down;".to_owned());

    object_fit
}