use std::collections::HashMap;


pub(crate) fn border_collapse() -> HashMap<String, String> {
    let mut border_collapse = HashMap::new();
    let _ = border_collapse.insert("border-collapse".to_owned(), "border-collapse: collapse;".to_owned());
    let _ = border_collapse.insert("border-separate".to_owned(), "border-collapse: separate;".to_owned());

    border_collapse
}