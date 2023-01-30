use std::collections::HashMap;


pub(crate) fn aspect_ratio() -> HashMap<String, String> {
    let mut aspect = HashMap::new();
    let _ = aspect.insert("aspect-auto".to_owned(), "aspect-ratio: auto;".to_owned());
    let _ = aspect.insert("aspect-square".to_owned(), "aspect-ratio: 1 / 1;".to_owned());
    let _ = aspect.insert("aspect-video".to_owned(), "aspect-ratio: 16 / 9;".to_owned());

    aspect
}