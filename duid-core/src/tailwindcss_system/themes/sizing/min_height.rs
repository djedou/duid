use std::collections::HashMap;


pub(crate) fn min_height() -> HashMap<String, String> {
    let mut min_height = HashMap::new();
    let _ = min_height.insert("min-h-0".to_owned(), "min-height: 0px;".to_owned());
    let _ = min_height.insert("min-h-full".to_owned(), "min-height: 100%;".to_owned());
    let _ = min_height.insert("min-h-screen".to_owned(), "min-height: 100vh;".to_owned());
    let _ = min_height.insert("min-h-min".to_owned(), "min-height: min-content;".to_owned());
    let _ = min_height.insert("min-h-max".to_owned(), "min-height: max-content;".to_owned());
    let _ = min_height.insert("min-h-fit".to_owned(), "min-height: fit-content;".to_owned());
    let _ = min_height.insert("min-vh-25".to_owned(), "min-height: 25vh;".to_owned());
    let _ = min_height.insert("min-vh-50".to_owned(), "min-height: 50vh;".to_owned());
    let _ = min_height.insert("min-vh-75".to_owned(), "min-height: 75vh;".to_owned());
    let _ = min_height.insert("min-vh-100".to_owned(), "min-height: 100vh;".to_owned());

    min_height
}