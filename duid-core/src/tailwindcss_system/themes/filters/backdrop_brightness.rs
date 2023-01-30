use std::collections::HashMap;


pub(crate) fn backdrop_brightness() -> HashMap<String, String> {
    let mut backdrop_brightness = HashMap::new();
    let _ = backdrop_brightness.insert("backdrop-brightness-0".to_owned(), "backdrop-filter: brightness(0);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-50".to_owned(), "backdrop-filter: brightness(.5);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-75".to_owned(), "backdrop-filter: brightness(.75);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-90".to_owned(), "backdrop-filter: brightness(.9);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-95".to_owned(), "backdrop-filter: brightness(.95);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-100".to_owned(), "backdrop-filter: brightness(1);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-105".to_owned(), "backdrop-filter: brightness(1.05);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-110".to_owned(), "backdrop-filter: brightness(1.1);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-125".to_owned(), "backdrop-filter: brightness(1.25);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-150".to_owned(), "backdrop-filter: brightness(1.5);".to_owned());
    let _ = backdrop_brightness.insert("backdrop-brightness-200".to_owned(), "backdrop-filter: brightness(2);".to_owned());

    backdrop_brightness
}