use std::collections::HashMap;


pub(crate) fn drop_shadow() -> HashMap<String, String> {
    let mut drop_shadow = HashMap::new();
    let _ = drop_shadow.insert("drop-shadow-sm".to_owned(), "filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));".to_owned());
    let _ = drop_shadow.insert("drop-shadow".to_owned(), "filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));".to_owned());
    let _ = drop_shadow.insert("drop-shadow-md".to_owned(), "filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));".to_owned());
    let _ = drop_shadow.insert("drop-shadow-lg".to_owned(), "filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));".to_owned());
    let _ = drop_shadow.insert("drop-shadow-xl".to_owned(), "filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));".to_owned());
    let _ = drop_shadow.insert("drop-shadow-2xl".to_owned(), "filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));".to_owned());
    let _ = drop_shadow.insert("drop-shadow-none".to_owned(), "filter: drop-shadow(0 0 #0000);".to_owned());

    drop_shadow
}