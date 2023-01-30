use std::collections::HashMap;


pub(crate) fn text_decoration_thickness() -> HashMap<String, String> {
    let mut text_decoration_thickness = HashMap::new();
    let _ = text_decoration_thickness.insert("decoration-auto".to_owned(), "text-decoration-thickness: auto;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-from-font".to_owned(), "text-decoration-thickness: from-font;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-0".to_owned(), "text-decoration-thickness: 0px;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-1".to_owned(), "text-decoration-thickness: 1px;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-2".to_owned(), "text-decoration-thickness: 2px;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-4".to_owned(), "text-decoration-thickness: 4px;".to_owned());
    let _ = text_decoration_thickness.insert("decoration-8".to_owned(), "text-decoration-thickness: 8px;".to_owned());

    text_decoration_thickness
}