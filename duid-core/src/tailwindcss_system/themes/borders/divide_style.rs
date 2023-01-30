use std::collections::HashMap;


pub(crate) fn divide_style() -> HashMap<String, String> {
    let mut divide_style = HashMap::new();
    let _ = divide_style.insert("divide-solid".to_owned(), "border-style: solid;".to_owned());
    let _ = divide_style.insert("divide-dashed".to_owned(), "border-style: dashed;".to_owned());
    let _ = divide_style.insert("divide-dotted".to_owned(), "border-style: dotted;".to_owned());
    let _ = divide_style.insert("divide-double".to_owned(), "border-style: double;".to_owned());
    let _ = divide_style.insert("divide-none".to_owned(), "border-style: none;".to_owned());

    divide_style
}