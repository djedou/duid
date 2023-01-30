use std::collections::HashMap;


pub(crate) fn backdrop_hue_rotate() -> HashMap<String, String> {
    let mut backdrop_hue_rotate = HashMap::new();
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-0".to_owned(), "backdrop-filter: hue-rotate(0deg);".to_owned());
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-15".to_owned(), "backdrop-filter: hue-rotate(15deg);".to_owned());
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-30".to_owned(), "backdrop-filter: hue-rotate(30deg);".to_owned());
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-60".to_owned(), "backdrop-filter: hue-rotate(60deg);".to_owned());
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-90".to_owned(), "backdrop-filter: hue-rotate(90deg);".to_owned());
    let _ = backdrop_hue_rotate.insert("backdrop-hue-rotate-180".to_owned(), "backdrop-filter: hue-rotate(180deg);".to_owned());

    backdrop_hue_rotate
}