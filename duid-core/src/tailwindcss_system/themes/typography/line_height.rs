use std::collections::HashMap;


pub(crate) fn line_height() -> HashMap<String, String> {
    let mut line_height = HashMap::new();
    let _ = line_height.insert("leading-3".to_owned(), "line-height: .75rem;".to_owned()); /* 12px */
    let _ = line_height.insert("leading-4".to_owned(), "line-height: 1rem;".to_owned()); /* 16px */
    let _ = line_height.insert("leading-5".to_owned(), "line-height: 1.25rem;".to_owned()); /* 20px */
    let _ = line_height.insert("leading-6".to_owned(), "line-height: 1.5rem;".to_owned()); /* 24px */
    let _ = line_height.insert("leading-7".to_owned(), "line-height: 1.75rem;".to_owned()); /* 28px */
    let _ = line_height.insert("leading-8".to_owned(), "line-height: 2rem;".to_owned()); /* 32px */
    let _ = line_height.insert("leading-9".to_owned(), "line-height: 2.25rem;".to_owned()); /* 36px */
    let _ = line_height.insert("leading-10".to_owned(), "line-height: 2.5rem;".to_owned()); /* 40px */
    let _ = line_height.insert("leading-none".to_owned(), "line-height: 1;".to_owned());
    let _ = line_height.insert("leading-tight".to_owned(), "line-height: 1.25;".to_owned());
    let _ = line_height.insert("leading-snug".to_owned(), "line-height: 1.375;".to_owned());
    let _ = line_height.insert("leading-normal".to_owned(), "line-height: 1.5;".to_owned());
    let _ = line_height.insert("leading-relaxed".to_owned(), "line-height: 1.625;".to_owned());
    let _ = line_height.insert("leading-loose".to_owned(), "line-height: 2;".to_owned());

    line_height
}