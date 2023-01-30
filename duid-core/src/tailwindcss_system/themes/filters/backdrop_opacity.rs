use std::collections::HashMap;


pub(crate) fn backdrop_opacity() -> HashMap<String, String> {
    let mut backdrop_opacity = HashMap::new();
    let _ = backdrop_opacity.insert("backdrop-opacity-0".to_owned(), "backdrop-filter: opacity(0);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-5".to_owned(), "backdrop-filter: opacity(0.05);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-10".to_owned(), "backdrop-filter: opacity(0.1);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-20".to_owned(), "backdrop-filter: opacity(0.2);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-25".to_owned(), "backdrop-filter: opacity(0.25);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-30".to_owned(), "backdrop-filter: opacity(0.3);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-40".to_owned(), "backdrop-filter: opacity(0.4);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-50".to_owned(), "backdrop-filter: opacity(0.5);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-60".to_owned(), "backdrop-filter: opacity(0.6);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-70".to_owned(), "backdrop-filter: opacity(0.7);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-75".to_owned(), "backdrop-filter: opacity(0.75);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-80".to_owned(), "backdrop-filter: opacity(0.8);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-90".to_owned(), "backdrop-filter: opacity(0.9);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-95".to_owned(), "backdrop-filter: opacity(0.95);".to_owned());
    let _ = backdrop_opacity.insert("backdrop-opacity-100".to_owned(), "backdrop-filter: opacity(1);".to_owned());

    backdrop_opacity
}