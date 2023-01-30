use std::collections::HashMap;


pub(crate) fn flex() -> HashMap<String, String> {
    let mut flex = HashMap::new();
    let _ = flex.insert("flex-1".to_owned(), "flex: 1 1 0%;".to_owned());
    let _ = flex.insert("flex-auto".to_owned(), "flex: 1 1 auto;".to_owned());
    let _ = flex.insert("flex-initial".to_owned(), "flex: 0 1 auto;".to_owned());
    let _ = flex.insert("flex-none".to_owned(), "flex: none;".to_owned());

    flex
}