use std::collections::HashMap;


pub(crate) fn isolation() -> HashMap<String, String> {
    let mut isolation = HashMap::new();
    let _ = isolation.insert("isolate".to_owned(), "isolation: isolate;".to_owned());
    let _ = isolation.insert("isolation-auto".to_owned(), "isolation: auto;".to_owned());

    isolation
}