use std::collections::HashMap;


pub(crate) fn font_style() -> HashMap<String, String> {
    let mut font_style = HashMap::new();
    let _ = font_style.insert("italic".to_owned(), "font-style: italic;".to_owned());
    let _ = font_style.insert("not-italic".to_owned(), "font-style: normal;".to_owned());

    font_style
}
