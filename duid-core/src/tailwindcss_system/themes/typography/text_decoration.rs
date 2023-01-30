use std::collections::HashMap;


pub(crate) fn text_decoration() -> HashMap<String, String> {
    let mut text_decoration = HashMap::new();
    let _ = text_decoration.insert("underline".to_owned(), "text-decoration-line: underline;".to_owned());
    let _ = text_decoration.insert("overline".to_owned(), "text-decoration-line: overline;".to_owned());
    let _ = text_decoration.insert("line-through".to_owned(), "text-decoration-line: line-through;".to_owned());
    let _ = text_decoration.insert("no-underline".to_owned(), "text-decoration-line: none;".to_owned());

    text_decoration
}