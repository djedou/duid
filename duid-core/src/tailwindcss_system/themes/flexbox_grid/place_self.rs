use std::collections::HashMap;


pub(crate) fn place_self() -> HashMap<String, String> {
    let mut place_self = HashMap::new();
    let _ = place_self.insert("place-self-auto".to_owned(), "place-self: auto;".to_owned());
    let _ = place_self.insert("place-self-start".to_owned(), "place-self: start;".to_owned());
    let _ = place_self.insert("place-self-end".to_owned(), "place-self: end;".to_owned());
    let _ = place_self.insert("place-self-center".to_owned(), "place-self: center;".to_owned());
    let _ = place_self.insert("place-self-stretch".to_owned(), "place-self: stretch;".to_owned());

    place_self
}