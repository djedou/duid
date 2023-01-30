use std::collections::HashMap;


pub(crate) fn word_break() -> HashMap<String, String> {
    let mut word_break = HashMap::new();
    let _ = word_break.insert("break-normal".to_owned(), "overflow-wrap: normal;word-break: normal;".to_owned());
    let _ = word_break.insert("break-words".to_owned(), "overflow-wrap: break-word;".to_owned());
    let _ = word_break.insert("break-all".to_owned(), "word-break: break-all;".to_owned());
    let _ = word_break.insert("break-keep".to_owned(), "word-break: keep-all;".to_owned());

    word_break
}
