use std::collections::HashMap;


pub(crate) fn border_style() -> HashMap<String, String> {
    let mut border_style = HashMap::new();
    let _ = border_style.insert("border-solid".to_owned(), "border-style: solid;".to_owned());
    let _ = border_style.insert("border-dashed".to_owned(), "border-style: dashed;".to_owned());
    let _ = border_style.insert("border-dotted".to_owned(), "border-style: dotted;".to_owned());
    let _ = border_style.insert("border-double".to_owned(), "border-style: double;".to_owned());
    let _ = border_style.insert("border-hidden".to_owned(), "border-style: hidden;".to_owned());
    let _ = border_style.insert("border-none".to_owned(), "border-style: none;".to_owned());

    border_style
}