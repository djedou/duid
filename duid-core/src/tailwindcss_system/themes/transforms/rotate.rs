use std::collections::HashMap;


pub(crate) fn rotate() -> HashMap<String, String> {
    let mut rotate = HashMap::new();
    let _ = rotate.insert("rotate-0".to_owned(), "transform: rotate(0deg);".to_owned());
    let _ = rotate.insert("rotate-1".to_owned(), "transform: rotate(1deg);".to_owned());
    let _ = rotate.insert("rotate-2".to_owned(), "transform: rotate(2deg);".to_owned());
    let _ = rotate.insert("rotate-3".to_owned(), "transform: rotate(3deg);".to_owned());
    let _ = rotate.insert("rotate-6".to_owned(), "transform: rotate(6deg);".to_owned());
    let _ = rotate.insert("rotate-12".to_owned(), "transform: rotate(12deg);".to_owned());
    let _ = rotate.insert("rotate-45".to_owned(), "transform: rotate(45deg);".to_owned());
    let _ = rotate.insert("rotate-90".to_owned(), "transform: rotate(90deg);".to_owned());
    let _ = rotate.insert("rotate-180".to_owned(), "transform: rotate(180deg);".to_owned());

    rotate
}