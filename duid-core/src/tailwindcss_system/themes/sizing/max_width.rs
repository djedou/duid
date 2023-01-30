use std::collections::HashMap;


pub(crate) fn max_width() -> HashMap<String, String> {
    let mut max_width = HashMap::new();
    let _ = max_width.insert("max-w-none".to_owned(), "max-width: none;".to_owned());
    let _ = max_width.insert("max-w-0".to_owned(), "max-width: 0rem;".to_owned()); /* 0px */
    let _ = max_width.insert("max-w-xs".to_owned(), "max-width: 20rem;".to_owned()); /* 320px */
    let _ = max_width.insert("max-w-sm".to_owned(), "max-width: 24rem;".to_owned()); /* 384px */
    let _ = max_width.insert("max-w-md".to_owned(), "max-width: 28rem;".to_owned()); /* 448px */
    let _ = max_width.insert("max-w-lg".to_owned(), "max-width: 32rem;".to_owned()); /* 512px */
    let _ = max_width.insert("max-w-xl".to_owned(), "max-width: 36rem;".to_owned()); /* 576px */
    let _ = max_width.insert("max-w-2xl".to_owned(), "max-width: 42rem;".to_owned()); /* 672px */
    let _ = max_width.insert("max-w-3xl".to_owned(), "max-width: 48rem;".to_owned()); /* 768px */
    let _ = max_width.insert("max-w-4xl".to_owned(), "max-width: 56rem;".to_owned()); /* 896px */
    let _ = max_width.insert("max-w-5xl".to_owned(), "max-width: 64rem;".to_owned()); /* 1024px */
    let _ = max_width.insert("max-w-6xl".to_owned(), "max-width: 72rem;".to_owned()); /* 1152px */
    let _ = max_width.insert("max-w-7xl".to_owned(), "max-width: 80rem;".to_owned()); /* 1280px */
    let _ = max_width.insert("max-w-full".to_owned(), "max-width: 100%;".to_owned());
    let _ = max_width.insert("max-w-min".to_owned(), "max-width: min-content;".to_owned());
    let _ = max_width.insert("max-w-max".to_owned(), "max-width: max-content;".to_owned());
    let _ = max_width.insert("max-w-fit".to_owned(), "max-width: fit-content;".to_owned());
    let _ = max_width.insert("max-w-prose".to_owned(), "max-width: 65ch;".to_owned());
    let _ = max_width.insert("max-w-screen-sm".to_owned(), "max-width: 640px;".to_owned());
    let _ = max_width.insert("max-w-screen-md".to_owned(), "max-width: 768px;".to_owned());
    let _ = max_width.insert("max-w-screen-lg".to_owned(), "max-width: 1024px;".to_owned());
    let _ = max_width.insert("max-w-screen-xl".to_owned(), "max-width: 1280px;".to_owned());
    let _ = max_width.insert("max-w-screen-2xl".to_owned(), "max-width: 1536px;".to_owned());
    let _ = max_width.insert("max-vw-25".to_owned(), "max-width: 25vw;".to_owned());
    let _ = max_width.insert("max-vw-50".to_owned(), "max-width: 50vw;".to_owned());
    let _ = max_width.insert("max-vw-75".to_owned(), "max-width: 75vw;".to_owned());
    let _ = max_width.insert("max-vw-100".to_owned(), "max-width: 100vw;".to_owned());

    max_width
}