use std::collections::HashMap;


pub(crate) fn letter_spacing() -> HashMap<String, String> {
    let mut letter_spacing = HashMap::new();
    let _ = letter_spacing.insert("tracking-tighter".to_owned(), "letter-spacing: -0.05em;".to_owned());
    let _ = letter_spacing.insert("tracking-tight".to_owned(), "letter-spacing: -0.025em;".to_owned());
    let _ = letter_spacing.insert("tracking-normal".to_owned(), "letter-spacing: 0em;".to_owned());
    let _ = letter_spacing.insert("tracking-wide".to_owned(), "letter-spacing: 0.025em;".to_owned());
    let _ = letter_spacing.insert("tracking-wider".to_owned(), "letter-spacing: 0.05em;".to_owned());
    let _ = letter_spacing.insert("tracking-widest".to_owned(), "letter-spacing: 0.1em;".to_owned());

    letter_spacing
}