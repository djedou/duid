use std::collections::HashMap;


pub(crate) fn box_shadow() -> HashMap<String, String> {
    let mut box_shadow = HashMap::new();
    let _ = box_shadow.insert("shadow-sm".to_owned(), "box-shadow: 0 1px 2px 0 var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow".to_owned(), "box-shadow: 0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-md".to_owned(), "box-shadow: 0 4px 6px -1px var(--tw-shadow-color), 0 2px 4px -2px var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-lg".to_owned(), "box-shadow: 0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-xl".to_owned(), "box-shadow: 0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-2xl".to_owned(), "box-shadow: 0 25px 50px -12px var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-inner".to_owned(), "box-shadow: inset 0 2px 4px 0 var(--tw-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-none".to_owned(), "box-shadow: 0 0 #0000;".to_owned());

    box_shadow
}