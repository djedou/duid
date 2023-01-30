
use std::collections::HashMap;


pub(crate) fn will_change() -> HashMap<String, String> {
    let mut will_change = HashMap::new();
    let _ = will_change.insert("will-change-auto".to_owned(), "will-change: auto;".to_owned());
    let _ = will_change.insert("will-change-scroll".to_owned(), "will-change: scroll-position;".to_owned());
    let _ = will_change.insert("will-change-contents".to_owned(), "will-change: contents;".to_owned());
    let _ = will_change.insert("will-change-transform".to_owned(), "will-change: transform;".to_owned());

    will_change
}