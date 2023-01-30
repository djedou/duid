use std::collections::HashMap;


pub(crate) fn flex_shrink() -> HashMap<String, String> {
    let mut flex_shrink = HashMap::new();
    let _ = flex_shrink.insert("shrink".to_owned(), "flex-shrink: 1;".to_owned());
    let _ = flex_shrink.insert("shrink-0".to_owned(), "flex-shrink: 0;".to_owned());

    flex_shrink
}