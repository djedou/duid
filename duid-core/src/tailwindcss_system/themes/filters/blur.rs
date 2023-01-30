
use std::collections::HashMap;


pub(crate) fn blur() -> HashMap<String, String> {
    let mut blur = HashMap::new();
    let _ = blur.insert("blur-none".to_owned(), "filter: blur(0);".to_owned());
    let _ = blur.insert("blur-sm".to_owned(), "filter: blur(4px);".to_owned());
    let _ = blur.insert("blur".to_owned(), "filter: blur(8px);".to_owned());
    let _ = blur.insert("blur-md".to_owned(), "filter: blur(12px);".to_owned());
    let _ = blur.insert("blur-lg".to_owned(), "filter: blur(16px);".to_owned());
    let _ = blur.insert("blur-xl".to_owned(), "filter: blur(24px);".to_owned());
    let _ = blur.insert("blur-2xl".to_owned(), "filter: blur(40px);".to_owned());
    let _ = blur.insert("blur-3xl".to_owned(), "filter: blur(64px);".to_owned());

    blur
}