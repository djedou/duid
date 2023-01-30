use std::collections::HashMap;


pub(crate) fn transition_duration() -> HashMap<String, String> {
    let mut transition_duration = HashMap::new();
    let _ = transition_duration.insert("duration-75".to_owned(), "transition-duration: 75ms;".to_owned());
    let _ = transition_duration.insert("duration-100".to_owned(), "transition-duration: 100ms;".to_owned());
    let _ = transition_duration.insert("duration-150".to_owned(), "transition-duration: 150ms;".to_owned());
    let _ = transition_duration.insert("duration-200".to_owned(), "transition-duration: 200ms;".to_owned());
    let _ = transition_duration.insert("duration-300".to_owned(), "transition-duration: 300ms;".to_owned());
    let _ = transition_duration.insert("duration-500".to_owned(), "transition-duration: 500ms;".to_owned());
    let _ = transition_duration.insert("duration-700".to_owned(), "transition-duration: 700ms;".to_owned());
    let _ = transition_duration.insert("duration-1000".to_owned(), "transition-duration: 1000ms;".to_owned());

    transition_duration
}