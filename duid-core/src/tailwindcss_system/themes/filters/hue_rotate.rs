use std::collections::HashMap;


pub(crate) fn hue_rotate() -> HashMap<String, String> {
    let mut hue_rotate = HashMap::new();
    let _ = hue_rotate.insert("hue-rotate-0".to_owned(), "filter: hue-rotate(0deg);".to_owned());
    let _ = hue_rotate.insert("hue-rotate-15".to_owned(), "filter: hue-rotate(15deg);".to_owned());
    let _ = hue_rotate.insert("hue-rotate-30".to_owned(), "filter: hue-rotate(30deg);".to_owned());
    let _ = hue_rotate.insert("hue-rotate-60".to_owned(), "filter: hue-rotate(60deg);".to_owned());
    let _ = hue_rotate.insert("hue-rotate-90".to_owned(), "filter: hue-rotate(90deg);".to_owned());
    let _ = hue_rotate.insert("hue-rotate-180".to_owned(), "filter: hue-rotate(180deg);".to_owned());

    hue_rotate
}