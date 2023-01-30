use std::collections::HashMap;


pub(crate) fn min_width() -> HashMap<String, String> {
    let mut min_width = HashMap::new();
    let _ = min_width.insert("min-w-0".to_owned(), "min-width: 0px;".to_owned());
    let _ = min_width.insert("min-w-full".to_owned(), "min-width: 100%;".to_owned());
    let _ = min_width.insert("min-w-min".to_owned(), "min-width: min-content;".to_owned());
    let _ = min_width.insert("min-w-max".to_owned(), "min-width: max-content;".to_owned());
    let _ = min_width.insert("min-w-fit".to_owned(), "min-width: fit-content;".to_owned());
    let _ = min_width.insert("min-vw-25".to_owned(), "min-width: 25vw;".to_owned());
    let _ = min_width.insert("min-vw-50".to_owned(), "min-width: 50vw;".to_owned());
    let _ = min_width.insert("min-vw-75".to_owned(), "min-width: 75vw;".to_owned());
    let _ = min_width.insert("min-vw-100".to_owned(), "min-width: 100vw;".to_owned());
    
    min_width
}