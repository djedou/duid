use std::collections::HashMap;


pub(crate) fn scroll_snap_align() -> HashMap<String, String> {
    let mut scroll_snap_align = HashMap::new();
    let _ = scroll_snap_align.insert("snap-start".to_owned(), "scroll-snap-align: start;".to_owned());
    let _ = scroll_snap_align.insert("snap-end".to_owned(), "scroll-snap-align: end;".to_owned());
    let _ = scroll_snap_align.insert("snap-center".to_owned(), "scroll-snap-align: center;".to_owned());
    let _ = scroll_snap_align.insert("snap-align-none".to_owned(), "scroll-snap-align: none;".to_owned());

    scroll_snap_align
}