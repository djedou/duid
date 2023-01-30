use std::collections::HashMap;


pub(crate) fn ring_offset_width() -> HashMap<String, String> {
    let mut ring_offset_width = HashMap::new();
    let _ = ring_offset_width.insert("ring-offset-0".to_owned(), "--tw-ring-offset-width: 0px;box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);".to_owned());
    let _ = ring_offset_width.insert("ring-offset-1".to_owned(), "--tw-ring-offset-width: 1px;box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);".to_owned());
    let _ = ring_offset_width.insert("ring-offset-2".to_owned(), "--tw-ring-offset-width: 2px;box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);".to_owned());
    let _ = ring_offset_width.insert("ring-offset-4".to_owned(), "--tw-ring-offset-width: 4px;box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);".to_owned());
    let _ = ring_offset_width.insert("ring-offset-8".to_owned(), "--tw-ring-offset-width: 8px;box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);".to_owned());

    ring_offset_width
}