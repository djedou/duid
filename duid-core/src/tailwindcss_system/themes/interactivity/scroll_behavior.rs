use std::collections::HashMap;


pub(crate) fn scroll_behavior() -> HashMap<String, String> {
    let mut scroll_behavior = HashMap::new();
    let _ = scroll_behavior.insert("scroll-auto".to_owned(), "scroll-behavior: auto;".to_owned());
    let _ = scroll_behavior.insert("scroll-smooth".to_owned(), "scroll-behavior: smooth;".to_owned());

    scroll_behavior
}