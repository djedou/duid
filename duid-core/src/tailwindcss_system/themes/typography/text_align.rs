use std::collections::HashMap;


pub(crate) fn text_align() -> HashMap<String, String> {
    let mut text_align = HashMap::new();
    let _ = text_align.insert("text-left".to_owned(), "text-align: left;".to_owned());
    let _ = text_align.insert("text-center".to_owned(), "text-align: center;".to_owned());
    let _ = text_align.insert("text-right".to_owned(), "text-align: right;".to_owned());
    let _ = text_align.insert("text-justify".to_owned(), "text-align: justify;".to_owned());
    let _ = text_align.insert("text-start".to_owned(), "text-align: start;".to_owned());
    let _ = text_align.insert("text-end".to_owned(), "text-align: end;".to_owned());

    text_align
}