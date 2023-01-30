use std::collections::HashMap;


pub(crate) fn transform_origin() -> HashMap<String, String> {
    let mut transform_origin = HashMap::new();
    let _ = transform_origin.insert("origin-center".to_owned(), "transform-origin: center;".to_owned());
    let _ = transform_origin.insert("origin-top".to_owned(), "transform-origin: top;".to_owned());
    let _ = transform_origin.insert("origin-top-right".to_owned(), "transform-origin: top right;".to_owned());
    let _ = transform_origin.insert("origin-right".to_owned(), "transform-origin: right;".to_owned());
    let _ = transform_origin.insert("origin-bottom-right".to_owned(), "transform-origin: bottom right;".to_owned());
    let _ = transform_origin.insert("origin-bottom".to_owned(), "transform-origin: bottom;".to_owned());
    let _ = transform_origin.insert("origin-bottom-left".to_owned(), "transform-origin: bottom left;".to_owned());
    let _ = transform_origin.insert("origin-left".to_owned(), "transform-origin: left;".to_owned());
    let _ = transform_origin.insert("origin-top-left".to_owned(), "transform-origin: top left;".to_owned());

    transform_origin
}