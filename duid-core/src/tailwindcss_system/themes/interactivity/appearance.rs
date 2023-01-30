use std::collections::HashMap;


pub(crate) fn appearance() -> HashMap<String, String> {
    let mut appearance = HashMap::new();
    let _ = appearance.insert("appearance-none".to_owned(), "appearance: none;".to_owned());

    appearance
}