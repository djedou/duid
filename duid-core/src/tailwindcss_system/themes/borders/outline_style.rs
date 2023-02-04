use std::collections::HashMap;


pub(crate) fn outline_style() -> HashMap<String, String> {
    let mut outline_style = HashMap::new();
    let _ = outline_style.insert("outline-none".to_owned(), "outline: 1px solid transparent;outline-offset: 1px;".to_owned());
    let _ = outline_style.insert("outline".to_owned(), "outline-style: solid;".to_owned());
    let _ = outline_style.insert("outline-dashed".to_owned(), "outline-style: dashed;".to_owned());
    let _ = outline_style.insert("outline-dotted".to_owned(), "outline-style: dotted;".to_owned());
    let _ = outline_style.insert("outline-double".to_owned(), "outline-style: double;".to_owned());

    outline_style
}