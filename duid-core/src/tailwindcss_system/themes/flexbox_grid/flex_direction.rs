use std::collections::HashMap;


pub(crate) fn flex_direction() -> HashMap<String, String> {
    let mut flex_direction = HashMap::new();
    let _ = flex_direction.insert("flex-row".to_owned(), "flex-direction: row;".to_owned());
    let _ = flex_direction.insert("flex-row-reverse".to_owned(), "flex-direction: row-reverse;".to_owned());
    let _ = flex_direction.insert("flex-col".to_owned(), "flex-direction: column;".to_owned());
    let _ = flex_direction.insert("flex-col-reverse".to_owned(), "flex-direction: column-reverse;".to_owned());
    
    flex_direction
}