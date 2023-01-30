use std::collections::HashMap;


pub(crate) fn scroll_snap_stop() -> HashMap<String, String> {
    let mut croll_snap_stop = HashMap::new();
    let _ = croll_snap_stop.insert("snap-normal".to_owned(), "scroll-snap-stop: normal;".to_owned());
    let _ = croll_snap_stop.insert("snap-always".to_owned(), "scroll-snap-stop: always;".to_owned());

    croll_snap_stop
}