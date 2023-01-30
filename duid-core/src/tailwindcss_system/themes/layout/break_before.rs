use std::collections::HashMap;


pub(crate) fn break_before() -> HashMap<String, String> {
    let mut break_before = HashMap::new();
    let _ = break_before.insert("break-before-auto".to_owned(), "break-before: auto;".to_owned());
    let _ = break_before.insert("break-before-avoid".to_owned(), "break-before: avoid;".to_owned());
    let _ = break_before.insert("break-before-all".to_owned(), "break-before: all;".to_owned());
    let _ = break_before.insert("break-before-avoid-page".to_owned(), "break-before: avoid-page;".to_owned());
    let _ = break_before.insert("break-before-page".to_owned(), "break-before: page;".to_owned());
    let _ = break_before.insert("break-before-left".to_owned(), "break-before: left;".to_owned());
    let _ = break_before.insert("break-before-right".to_owned(), "break-before: right;".to_owned());
    let _ = break_before.insert("break-before-column".to_owned(), "break-before: column;".to_owned());
    
    

    break_before
}
