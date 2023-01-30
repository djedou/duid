use std::collections::HashMap;


pub(crate) fn transition_property() -> HashMap<String, String> {
    let mut transition_property = HashMap::new();
    let _ = transition_property.insert("transition-none".to_owned(), "transition-property: none;".to_owned());
    let _ = transition_property.insert("transition-all".to_owned(), "transition-property: all;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());
    let _ = transition_property.insert("transition".to_owned(), "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());
    let _ = transition_property.insert("transition-colors".to_owned(), "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());
    let _ = transition_property.insert("transition-opacity".to_owned(), "transition-property: opacity;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());
    let _ = transition_property.insert("transition-shadow".to_owned(), "transition-property: box-shadow;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());
    let _ = transition_property.insert("transition-transform".to_owned(), "transition-property: transform;transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);transition-duration: 150ms;".to_owned());

    transition_property
}