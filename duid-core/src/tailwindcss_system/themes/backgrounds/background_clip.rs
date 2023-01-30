use std::collections::HashMap;


pub(crate) fn background_clip() -> HashMap<String, String> {
    let mut background_clip = HashMap::new();
    let _ = background_clip.insert("bg-clip-border".to_owned(), "background-clip: border-box;".to_owned());
    let _ = background_clip.insert("bg-clip-padding".to_owned(), "background-clip: padding-box;".to_owned());
    let _ = background_clip.insert("bg-clip-content".to_owned(), "background-clip: content-box;".to_owned());
    let _ = background_clip.insert("bg-clip-text".to_owned(), "background-clip: text;".to_owned());

    background_clip
}