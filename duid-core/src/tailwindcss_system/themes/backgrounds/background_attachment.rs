use std::collections::HashMap;


pub(crate) fn background_attachment() -> HashMap<String, String> {
    let mut background_attachment = HashMap::new();
    let _ = background_attachment.insert("bg-fixed".to_owned(), "background-attachment: fixed;".to_owned());
    let _ = background_attachment.insert("bg-local".to_owned(), "background-attachment: local;".to_owned());
    let _ = background_attachment.insert("bg-scroll".to_owned(), "background-attachment: scroll;".to_owned());

    background_attachment
}