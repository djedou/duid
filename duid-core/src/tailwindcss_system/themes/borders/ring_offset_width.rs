use std::collections::HashMap;


pub(crate) fn ring_offset_width() -> HashMap<String, String> {
    let mut ring_offset_width = HashMap::new();
    let _ = ring_offset_width.insert("ring-offset-0".to_owned(), "--duid-ring-offset-width: 0px;".to_owned());
    let _ = ring_offset_width.insert("ring-offset-1".to_owned(), "--duid-ring-offset-width: 1px;".to_owned());
    let _ = ring_offset_width.insert("ring-offset-2".to_owned(), "--duid-ring-offset-width: 2px;".to_owned());
    let _ = ring_offset_width.insert("ring-offset-4".to_owned(), "--duid-ring-offset-width: 4px;".to_owned());
    let _ = ring_offset_width.insert("ring-offset-8".to_owned(), "--duid-ring-offset-width: 8px;".to_owned());

    ring_offset_width
}