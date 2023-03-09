use std::collections::HashMap;


pub(crate) fn ring_style() -> HashMap<String, String> {
    let mut ring = HashMap::new();
    let _ = ring.insert("ring-0".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(0px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    let _ = ring.insert("ring-1".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(1px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    let _ = ring.insert("ring-2".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(2px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    let _ = ring.insert("ring".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(3px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    let _ = ring.insert("ring-4".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(4px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    let _ = ring.insert("ring-8".to_owned(), "box-shadow: var(--duid-ring-inset) 0 0 0 calc(8px + var(--duid-ring-offset-width)) var(--duid-ring-color);".to_owned());
    
    let _ = ring.insert("ring-inset-var".to_owned(), "--duid-ring-inset: inset;".to_owned());

    ring
}