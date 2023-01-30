
use std::collections::HashMap;


pub(crate) fn list_style_type() -> HashMap<String, String> {
    let mut list_style_type = HashMap::new();
    let _ = list_style_type.insert("list-none".to_owned(), "list-style-type: none;".to_owned());
    let _ = list_style_type.insert("list-disc".to_owned(), "list-style-type: disc;".to_owned());
    let _ = list_style_type.insert("list-decimal".to_owned(), "list-style-type: decimal;".to_owned());

    list_style_type
}