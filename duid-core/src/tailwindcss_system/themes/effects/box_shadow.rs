use std::collections::HashMap;


pub(crate) fn box_shadow() -> HashMap<String, String> {
    let mut box_shadow = HashMap::new();
    let _ = box_shadow.insert("shadow-sm".to_owned(), "box-shadow: 0 1px 2px 0 var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow".to_owned(), "box-shadow: 0 1px 3px 0 var(--duid-shadow-color), 0 1px 2px -1px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-md".to_owned(), "box-shadow: 0 4px 6px -1px var(--duid-shadow-color), 0 2px 4px -2px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-lg".to_owned(), "box-shadow: 0 10px 15px -3px var(--duid-shadow-color), 0 4px 6px -4px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-xl".to_owned(), "box-shadow: 0 20px 25px -5px var(--duid-shadow-color), 0 8px 10px -6px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-2xl".to_owned(), "box-shadow: 0 25px 50px -12px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-inner".to_owned(), "box-shadow: inset 0 2px 4px 0 var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-none".to_owned(), "box-shadow: none;".to_owned());
    let _ = box_shadow.insert("shadow-btn-focus".to_owned(), "box-shadow: inset 0 0 0 3px var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-active".to_owned(), "box-shadow: var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-default".to_owned(), "box-shadow: var(--duid-shadow-color), var(--duid-inset-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-filled".to_owned(), "box-shadow: var(--duid-shadow-color), var(--duid-inset-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-filled-selected".to_owned(), "box-shadow: var(--duid-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-outline-hover".to_owned(), "box-shadow: var(--duid-shadow-color), var(----duid-inset-shadow-color);".to_owned());
    let _ = box_shadow.insert("shadow-btn-outline-selected".to_owned(), "box-shadow: var(--duid-shadow-color);".to_owned());
    
    box_shadow
}
