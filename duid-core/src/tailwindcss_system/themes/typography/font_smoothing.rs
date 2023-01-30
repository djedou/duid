use std::collections::HashMap;


pub(crate) fn font_smoothing() -> HashMap<String, String> {
    let mut font_smoothing = HashMap::new();
    let _ = font_smoothing.insert("antialiased".to_owned(), "-webkit-font-smoothing: antialiased;-moz-osx-font-smoothing: grayscale;".to_owned());
    let _ = font_smoothing.insert("subpixel-antialiased".to_owned(), "-webkit-font-smoothing: auto;-moz-osx-font-smoothing: auto;".to_owned());
    font_smoothing
}