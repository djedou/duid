use std::collections::HashMap;


pub(crate) fn transition_timing_function() -> HashMap<String, String> {
    let mut transition_timing_function = HashMap::new();
    let _ = transition_timing_function.insert("ease-linear".to_owned(), "transition-timing-function: linear;".to_owned());
    let _ = transition_timing_function.insert("ease-in".to_owned(), "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);".to_owned());
    let _ = transition_timing_function.insert("ease-out".to_owned(), "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);".to_owned());
    let _ = transition_timing_function.insert("ease-in-out".to_owned(), "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);".to_owned());

    transition_timing_function
}