use std::collections::HashMap;


pub(crate) fn justify_self() -> HashMap<String, String> {
    let mut justify_self = HashMap::new();
    let _ = justify_self.insert("justify-self-auto".to_owned(), "justify-self: auto;".to_owned());
    let _ = justify_self.insert("justify-self-start".to_owned(), "justify-self: start;".to_owned());
    let _ = justify_self.insert("justify-self-end".to_owned(), "justify-self: end;".to_owned());
    let _ = justify_self.insert("justify-self-center".to_owned(), "justify-self: center;".to_owned());
    let _ = justify_self.insert("justify-self-stretch".to_owned(), "justify-self: stretch;".to_owned());

    justify_self
}