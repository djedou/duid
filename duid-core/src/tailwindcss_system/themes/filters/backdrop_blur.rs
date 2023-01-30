use std::collections::HashMap;


pub(crate) fn backdrop_blur() -> HashMap<String, String> {
    let mut backdrop_blur = HashMap::new();
    let _ = backdrop_blur.insert("backdrop-blur-none".to_owned(), "backdrop-filter: blur(0);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-sm".to_owned(), "backdrop-filter: blur(4px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur".to_owned(), "backdrop-filter: blur(8px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-md".to_owned(), "backdrop-filter: blur(12px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-lg".to_owned(), "backdrop-filter: blur(16px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-xl".to_owned(), "backdrop-filter: blur(24px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-2xl".to_owned(), "backdrop-filter: blur(40px);".to_owned());
    let _ = backdrop_blur.insert("backdrop-blur-3xl".to_owned(), "backdrop-filter: blur(64px);".to_owned());

    backdrop_blur
}