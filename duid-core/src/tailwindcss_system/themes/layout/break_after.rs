use std::collections::HashMap;


pub(crate) fn break_after() -> HashMap<String, String> {
    let mut break_after = HashMap::new();
    let _ = break_after.insert("break-after-auto".to_owned(), "break-after: auto;".to_owned());
    let _ = break_after.insert("break-after-avoid".to_owned(), "break-after: avoid;".to_owned());
    let _ = break_after.insert("break-after-all".to_owned(), "break-after: all;".to_owned());
    let _ = break_after.insert("break-after-avoid-page".to_owned(), "break-after: avoid-page;".to_owned());
    let _ = break_after.insert("break-after-page".to_owned(), "break-after: page;".to_owned());
    let _ = break_after.insert("break-after-left".to_owned(), "break-after: left;".to_owned());
    let _ = break_after.insert("break-after-right".to_owned(), "break-after: right;".to_owned());
    let _ = break_after.insert("break-after-column".to_owned(), "break-after: column;".to_owned());
    

    break_after
}