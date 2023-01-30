use std::collections::HashMap;


pub(crate) fn place_content() -> HashMap<String, String> {
    let mut place_content = HashMap::new();
    let _ = place_content.insert("place-content-center".to_owned(), "place-content: center;".to_owned());
    let _ = place_content.insert("place-content-start".to_owned(), "place-content: start;".to_owned());
    let _ = place_content.insert("place-content-end".to_owned(), "place-content: end;".to_owned());
    let _ = place_content.insert("place-content-between".to_owned(), "place-content: space-between;".to_owned());
    let _ = place_content.insert("place-content-around".to_owned(), "place-content: space-around;".to_owned());
    let _ = place_content.insert("place-content-evenly".to_owned(), "place-content: space-evenly;".to_owned());
    let _ = place_content.insert("place-content-baseline".to_owned(), "place-content: baseline;".to_owned());
    let _ = place_content.insert("place-content-stretch".to_owned(), "place-content: stretch;".to_owned());

    place_content
}