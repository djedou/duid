use std::collections::HashMap;


pub(crate) fn scroll_snap_type() -> HashMap<String, String> {
    let mut scroll_snap_type = HashMap::new();
    let _ = scroll_snap_type.insert("snap-none".to_owned(), "scroll-snap-type: none;".to_owned());
    let _ = scroll_snap_type.insert("snap-x".to_owned(), "scroll-snap-type: x var(--tw-scroll-snap-strictness);".to_owned());
    let _ = scroll_snap_type.insert("snap-y".to_owned(), "scroll-snap-type: y var(--tw-scroll-snap-strictness);".to_owned());
    let _ = scroll_snap_type.insert("snap-both".to_owned(), "scroll-snap-type: both var(--tw-scroll-snap-strictness);".to_owned());
    let _ = scroll_snap_type.insert("snap-mandatory".to_owned(), "--tw-scroll-snap-strictness: mandatory;".to_owned());
    let _ = scroll_snap_type.insert("snap-proximity".to_owned(), "--tw-scroll-snap-strictness: proximity;".to_owned());

    scroll_snap_type
}