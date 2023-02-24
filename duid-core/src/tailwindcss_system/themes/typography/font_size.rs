use std::collections::HashMap;


pub(crate) fn font_size() -> HashMap<String, String> {
    let mut font_size = HashMap::new();
    let _ = font_size.insert("text-xs".to_owned(), "font-size: 0.75rem;".to_owned()); /* 16px */
    let _ = font_size.insert("text-sm".to_owned(), "font-size: 0.875rem;".to_owned()); /* 20px */
    let _ = font_size.insert("text-base".to_owned(), "font-size: 1rem;".to_owned()); /* 24px */
    let _ = font_size.insert("text-lg".to_owned(), "font-size: 1.125rem;".to_owned()); /* 28px */
    let _ = font_size.insert("text-xl".to_owned(), "font-size: 1.25rem;".to_owned()); /* 28px */
    let _ = font_size.insert("text-2xl".to_owned(), "font-size: 1.5rem;".to_owned()); /* 32px */
    let _ = font_size.insert("text-3xl".to_owned(), "font-size: 1.875rem;".to_owned()); /* 36px */
    let _ = font_size.insert("text-4xl".to_owned(), "font-size: 2.25rem;".to_owned()); /* 40px */
    let _ = font_size.insert("text-5xl".to_owned(), "font-size: 3rem;".to_owned());
    let _ = font_size.insert("text-6xl".to_owned(), "font-size: 3.75rem;".to_owned()); /* 60px */
    let _ = font_size.insert("text-7xl".to_owned(), "font-size: 4.5rem;".to_owned()); /* 72px */
    let _ = font_size.insert("text-8xl".to_owned(), "font-size: 6rem;".to_owned()); /* 96px */
    let _ = font_size.insert("text-9xl".to_owned(), "font-size: 8rem;".to_owned()); /* 128px */

    font_size
}
