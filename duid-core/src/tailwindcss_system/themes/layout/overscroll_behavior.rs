use std::collections::HashMap;


pub(crate) fn overscroll_behavior() -> HashMap<String, String> {
    let mut overscroll_behavior = HashMap::new();
    let _ = overscroll_behavior.insert("overscroll-auto".to_owned(), "overscroll-behavior: auto;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-contain".to_owned(), "overscroll-behavior: contain;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-none".to_owned(), "overscroll-behavior: none;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-y-auto".to_owned(), "overscroll-behavior-y: auto;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-y-contain".to_owned(), "overscroll-behavior-y: contain;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-y-none".to_owned(), "overscroll-behavior-y: none;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-x-auto".to_owned(), "overscroll-behavior-x: auto;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-x-contain".to_owned(), "overscroll-behavior-x: contain;".to_owned());
    let _ = overscroll_behavior.insert("overscroll-x-none".to_owned(), "overscroll-behavior-x: none;".to_owned());

    overscroll_behavior
}