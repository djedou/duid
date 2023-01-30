
use std::collections::HashMap;


pub(crate) fn text_decoration_style() -> HashMap<String, String> {
    let mut text_decoration_style = HashMap::new();
    let _ = text_decoration_style.insert("decoration-solid".to_owned(), "text-decoration-style: solid;".to_owned());
    let _ = text_decoration_style.insert("decoration-double".to_owned(), "text-decoration-style: double;".to_owned());
    let _ = text_decoration_style.insert("decoration-dotted".to_owned(), "text-decoration-style: dotted;".to_owned());
    let _ = text_decoration_style.insert("decoration-dashed".to_owned(), "text-decoration-style: dashed;".to_owned());
    let _ = text_decoration_style.insert("decoration-wavy".to_owned(), "text-decoration-style: wavy;".to_owned());

    text_decoration_style
}