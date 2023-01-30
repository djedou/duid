use std::collections::HashMap;


pub(crate) fn text_underline_offset() -> HashMap<String, String> {
    let mut text_underline_offset = HashMap::new();
    let _ = text_underline_offset.insert("underline-offset-auto".to_owned(), "text-underline-offset: auto;".to_owned());
    let _ = text_underline_offset.insert("underline-offset-0".to_owned(), "text-underline-offset: 0px;".to_owned());
    let _ = text_underline_offset.insert("underline-offset-1".to_owned(), "text-underline-offset: 1px;".to_owned());
    let _ = text_underline_offset.insert("underline-offset-2".to_owned(), "text-underline-offset: 2px;".to_owned());
    let _ = text_underline_offset.insert("underline-offset-4".to_owned(), "text-underline-offset: 4px;".to_owned());
    let _ = text_underline_offset.insert("underline-offset-8".to_owned(), "text-underline-offset: 8px;".to_owned());

    text_underline_offset
}