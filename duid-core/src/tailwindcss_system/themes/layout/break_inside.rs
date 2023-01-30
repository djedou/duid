use std::collections::HashMap;


pub(crate) fn break_inside() -> HashMap<String, String> {
    let mut break_inside = HashMap::new();
    let _ = break_inside.insert("break-inside-auto".to_owned(), "break-inside: auto;".to_owned());
    let _ = break_inside.insert("break-inside-avoid".to_owned(), "break-inside: avoid;".to_owned());
    let _ = break_inside.insert("break-inside-avoid-page".to_owned(), "break-inside: avoid-page;".to_owned());
    let _ = break_inside.insert("break-inside-avoid-column".to_owned(), "break-inside: avoid-column;".to_owned());
    

    break_inside
}