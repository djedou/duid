use std::collections::HashMap;


pub(crate) fn flex_wrap() -> HashMap<String, String> {
    let mut flex_wrap = HashMap::new();
    let _ = flex_wrap.insert("flex-wrap".to_owned(), "flex-wrap: wrap;".to_owned());
    let _ = flex_wrap.insert("flex-wrap-reverse".to_owned(), "flex-wrap: wrap-reverse;".to_owned());
    let _ = flex_wrap.insert("flex-nowrap".to_owned(), "flex-wrap: nowrap;".to_owned());

    flex_wrap
}
