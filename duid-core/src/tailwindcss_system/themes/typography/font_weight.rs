use std::collections::HashMap;


pub(crate) fn font_weight() -> HashMap<String, String> {
    let mut font_weight = HashMap::new();
    let _ = font_weight.insert("font-thin".to_owned(), "font-weight: 100;".to_owned());
    let _ = font_weight.insert("font-extralight".to_owned(), "font-weight: 200;".to_owned());
    let _ = font_weight.insert("font-light".to_owned(), "font-weight: 300;".to_owned());
    let _ = font_weight.insert("font-normal".to_owned(), "font-weight: 400;".to_owned());
    let _ = font_weight.insert("font-medium".to_owned(), "font-weight: 500;".to_owned());
    let _ = font_weight.insert("font-semibold".to_owned(), "font-weight: 600;".to_owned());
    let _ = font_weight.insert("font-bold".to_owned(), "font-weight: 700;".to_owned());
    let _ = font_weight.insert("font-extrabold".to_owned(), "font-weight: 800;".to_owned());
    let _ = font_weight.insert("font-black".to_owned(), "font-weight: 900;".to_owned());

    font_weight
}