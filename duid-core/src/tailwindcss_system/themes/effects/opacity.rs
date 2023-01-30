use std::collections::HashMap;


pub(crate) fn opacity() -> HashMap<String, String> {
    let mut opacity = HashMap::new();
    let _ = opacity.insert("opacity-0".to_owned(), "opacity: 0;".to_owned());
    let _ = opacity.insert("opacity-5".to_owned(), "opacity: 0.05;".to_owned());
    let _ = opacity.insert("opacity-10".to_owned(), "opacity: 0.1;".to_owned());
    let _ = opacity.insert("opacity-20".to_owned(), "opacity: 0.2;".to_owned());
    let _ = opacity.insert("opacity-25".to_owned(), "opacity: 0.25;".to_owned());
    let _ = opacity.insert("opacity-30".to_owned(), "opacity: 0.3;".to_owned());
    let _ = opacity.insert("opacity-40".to_owned(), "opacity: 0.4;".to_owned());
    let _ = opacity.insert("opacity-50".to_owned(), "opacity: 0.5;".to_owned());
    let _ = opacity.insert("opacity-60".to_owned(), "opacity: 0.6;".to_owned());
    let _ = opacity.insert("opacity-70".to_owned(), "opacity: 0.7;".to_owned());
    let _ = opacity.insert("opacity-75".to_owned(), "opacity: 0.75;".to_owned());
    let _ = opacity.insert("opacity-80".to_owned(), "opacity: 0.8;".to_owned());
    let _ = opacity.insert("opacity-90".to_owned(), "opacity: 0.9;".to_owned());
    let _ = opacity.insert("opacity-95".to_owned(), "opacity: 0.95;".to_owned());
    let _ = opacity.insert("opacity-100".to_owned(), "opacity: 1;".to_owned());

    opacity
}