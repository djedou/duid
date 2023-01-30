use std::collections::HashMap;


pub(crate) fn vertical_align() -> HashMap<String, String> {
    let mut vertical_align = HashMap::new();
    let _ = vertical_align.insert("align-baseline".to_owned(), "vertical-align: baseline;".to_owned());
    let _ = vertical_align.insert("align-top".to_owned(), "vertical-align: top;".to_owned());
    let _ = vertical_align.insert("align-middle".to_owned(), "vertical-align: middle;".to_owned());
    let _ = vertical_align.insert("align-bottom".to_owned(), "vertical-align: bottom;".to_owned());
    let _ = vertical_align.insert("align-text-top".to_owned(), "vertical-align: text-top;".to_owned());
    let _ = vertical_align.insert("align-text-bottom".to_owned(), "vertical-align: text-bottom;".to_owned());
    let _ = vertical_align.insert("align-sub".to_owned(), "vertical-align: sub;".to_owned());
    let _ = vertical_align.insert("align-super".to_owned(), "vertical-align: super;".to_owned());

    vertical_align
}