use std::collections::HashMap;


pub(crate) fn brightness() -> HashMap<String, String> {
    let mut brightness = HashMap::new();
    let _ = brightness.insert("brightness-0".to_owned(), "filter: brightness(0);".to_owned());
    let _ = brightness.insert("brightness-50".to_owned(), "filter: brightness(.5);".to_owned());
    let _ = brightness.insert("brightness-75".to_owned(), "filter: brightness(.75);".to_owned());
    let _ = brightness.insert("brightness-90".to_owned(), "filter: brightness(.9);".to_owned());
    let _ = brightness.insert("brightness-95".to_owned(), "filter: brightness(.95);".to_owned());
    let _ = brightness.insert("brightness-100".to_owned(), "filter: brightness(1);".to_owned());
    let _ = brightness.insert("brightness-105".to_owned(), "filter: brightness(1.05);".to_owned());
    let _ = brightness.insert("brightness-110".to_owned(), "filter: brightness(1.1);".to_owned());
    let _ = brightness.insert("brightness-125".to_owned(), "filter: brightness(1.25);".to_owned());
    let _ = brightness.insert("brightness-150".to_owned(), "filter: brightness(1.5);".to_owned());
    let _ = brightness.insert("brightness-200".to_owned(), "filter: brightness(2);".to_owned());

    brightness
}