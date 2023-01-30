use std::collections::HashMap;


pub(crate) fn ring_width() -> HashMap<String, String> {
    let mut ring_width = HashMap::new();
    let _ = ring_width.insert("ring-0".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring-1".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring-2".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring-4".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring-8".to_owned(), "box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_owned());
    let _ = ring_width.insert("ring-inset".to_owned(), "--tw-ring-inset: inset;".to_owned());

    ring_width
}