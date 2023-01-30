use std::collections::HashMap;


pub(crate) fn skew() -> HashMap<String, String> {
    let mut skew = HashMap::new();
    let _ = skew.insert("skew-x-0".to_owned(), "transform: skewX(0deg);".to_owned());
    let _ = skew.insert("skew-y-0".to_owned(), "transform: skewY(0deg);".to_owned());
    let _ = skew.insert("skew-x-1".to_owned(), "transform: skewX(1deg);".to_owned());
    let _ = skew.insert("skew-y-1".to_owned(), "transform: skewY(1deg);".to_owned());
    let _ = skew.insert("skew-x-2".to_owned(), "transform: skewX(2deg);".to_owned());
    let _ = skew.insert("skew-y-2".to_owned(), "transform: skewY(2deg);".to_owned());
    let _ = skew.insert("skew-x-3".to_owned(), "transform: skewX(3deg);".to_owned());
    let _ = skew.insert("skew-y-3".to_owned(), "transform: skewY(3deg);".to_owned());
    let _ = skew.insert("skew-x-6".to_owned(), "transform: skewX(6deg);".to_owned());
    let _ = skew.insert("skew-y-6".to_owned(), "transform: skewY(6deg);".to_owned());
    let _ = skew.insert("skew-x-12".to_owned(), "transform: skewX(12deg);".to_owned());
    let _ = skew.insert("skew-y-12".to_owned(), "transform: skewY(12deg);".to_owned());

    skew
}