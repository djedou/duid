use std::collections::HashMap;


pub(crate) fn pointer_events() -> HashMap<String, String> {
    let mut pointer_events = HashMap::new();
    let _ = pointer_events.insert("pointer-events-none".to_owned(), "pointer-events: none;".to_owned());
    let _ = pointer_events.insert("pointer-events-auto".to_owned(), "pointer-events: auto;".to_owned());
    let _ = pointer_events.insert("pointer-events-all".to_owned(), "pointer-events: all;".to_owned());

    pointer_events
}