use std::collections::HashMap;


pub(crate) fn align_self() -> HashMap<String, String> {
    let mut align_self = HashMap::new();
    let _ = align_self.insert("self-auto".to_owned(), "align-self: auto;".to_owned());
    let _ = align_self.insert("self-start".to_owned(), "align-self: flex-start;".to_owned());
    let _ = align_self.insert("self-end".to_owned(), "align-self: flex-end;".to_owned());
    let _ = align_self.insert("self-center".to_owned(), "align-self: center;".to_owned());
    let _ = align_self.insert("self-stretch".to_owned(), "align-self: stretch;".to_owned());
    let _ = align_self.insert("self-baseline".to_owned(), "align-self: baseline;".to_owned());
    
    align_self
}