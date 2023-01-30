use std::collections::HashMap;


pub(crate) fn transition_delay() -> HashMap<String, String> {
    let mut transition_delay = HashMap::new();
    let _ = transition_delay.insert("delay-75".to_owned(), "transition-delay: 75ms;".to_owned());
    let _ = transition_delay.insert("delay-100".to_owned(), "transition-delay: 100ms;".to_owned());
    let _ = transition_delay.insert("delay-150".to_owned(), "transition-delay: 150ms;".to_owned());
    let _ = transition_delay.insert("delay-200".to_owned(), "transition-delay: 200ms;".to_owned());
    let _ = transition_delay.insert("delay-300".to_owned(), "transition-delay: 300ms;".to_owned());
    let _ = transition_delay.insert("delay-500".to_owned(), "transition-delay: 500ms;".to_owned());
    let _ = transition_delay.insert("delay-700".to_owned(), "transition-delay: 700ms;".to_owned());
    let _ = transition_delay.insert("delay-1000".to_owned(), "transition-delay: 1000ms;".to_owned());

    transition_delay
}