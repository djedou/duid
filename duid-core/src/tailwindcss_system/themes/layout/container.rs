use std::collections::HashMap;


pub(crate) fn container() -> HashMap<String, String> {
    let mut container = HashMap::new();
    let _ = container.insert("container-xs".to_owned(), "width: 100%;".to_owned());
    let _ = container.insert("container-sm".to_owned(), "max-width: 640px;".to_owned());
    let _ = container.insert("container-md".to_owned(), "max-width: 768px;".to_owned());
    let _ = container.insert("container-lg".to_owned(), "max-width: 1024px;".to_owned());
    let _ = container.insert("container-xl".to_owned(), "max-width: 1280px;".to_owned());
    let _ = container.insert("container-2xl".to_owned(), "max-width: 1536px;".to_owned());

    container
}