use std::collections::HashMap;


pub(crate) fn align_content() -> HashMap<String, String> {
    let mut align_content = HashMap::new();
    let _ = align_content.insert("content-center".to_owned(), "align-content: center;".to_owned());
    let _ = align_content.insert("content-start".to_owned(), "align-content: flex-start;".to_owned());
    let _ = align_content.insert("content-end".to_owned(), "align-content: flex-end;".to_owned());
    let _ = align_content.insert("content-between".to_owned(), "align-content: space-between;".to_owned());
    let _ = align_content.insert("content-around".to_owned(), "align-content: space-around;".to_owned());
    let _ = align_content.insert("content-evenly".to_owned(), "align-content: space-evenly;".to_owned());
    let _ = align_content.insert("content-baseline".to_owned(), "align-content: baseline;".to_owned());

    align_content
}