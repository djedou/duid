use std::collections::HashMap;


pub(crate) fn text_indent() -> HashMap<String, String> {
    let mut text_indent = HashMap::new();
    let _ = text_indent.insert("indent-0".to_owned(), "text-indent: 0px;".to_owned());
    let _ = text_indent.insert("indent-px".to_owned(), "text-indent: 1px;".to_owned());
    let _ = text_indent.insert("indent-0.5".to_owned(), "text-indent: 0.125rem;".to_owned()); /* 2px */
    let _ = text_indent.insert("indent-1".to_owned(), "text-indent: 0.25rem;".to_owned()); /* 4px */
    let _ = text_indent.insert("indent-1.5".to_owned(), "text-indent: 0.375rem;".to_owned()); /* 6px */
    let _ = text_indent.insert("indent-2".to_owned(), "text-indent: 0.5rem;".to_owned()); /* 8px */
    let _ = text_indent.insert("indent-2.5".to_owned(), "text-indent: 0.625rem;".to_owned()); /* 10px */
    let _ = text_indent.insert("indent-3".to_owned(), "text-indent: 0.75rem;".to_owned()); /* 12px */
    let _ = text_indent.insert("indent-3.5".to_owned(), "text-indent: 0.875rem;".to_owned()); /* 14px */
    let _ = text_indent.insert("indent-4".to_owned(), "text-indent: 1rem;".to_owned()); /* 16px */
    let _ = text_indent.insert("indent-5".to_owned(), "text-indent: 1.25rem;".to_owned()); /* 20px */
    let _ = text_indent.insert("indent-6".to_owned(), "text-indent: 1.5rem;".to_owned()); /* 24px */
    let _ = text_indent.insert("indent-7".to_owned(), "text-indent: 1.75rem;".to_owned()); /* 28px */
    let _ = text_indent.insert("indent-8".to_owned(), "text-indent: 2rem;".to_owned()); /* 32px */
    let _ = text_indent.insert("indent-9".to_owned(), "text-indent: 2.25rem;".to_owned()); /* 36px */
    let _ = text_indent.insert("indent-10".to_owned(), "text-indent: 2.5rem;".to_owned()); /* 40px */
    let _ = text_indent.insert("indent-11".to_owned(), "text-indent: 2.75rem;".to_owned()); /* 44px */
    let _ = text_indent.insert("indent-12".to_owned(), "text-indent: 3rem;".to_owned()); /* 48px */
    let _ = text_indent.insert("indent-14".to_owned(), "text-indent: 3.5rem;".to_owned()); /* 56px */
    let _ = text_indent.insert("indent-16".to_owned(), "text-indent: 4rem;".to_owned()); /* 64px */
    let _ = text_indent.insert("indent-20".to_owned(), "text-indent: 5rem;".to_owned()); /* 80px */
    let _ = text_indent.insert("indent-24".to_owned(), "text-indent: 6rem;".to_owned()); /* 96px */
    let _ = text_indent.insert("indent-28".to_owned(), "text-indent: 7rem;".to_owned()); /* 112px */
    let _ = text_indent.insert("indent-32".to_owned(), "text-indent: 8rem;".to_owned()); /* 128px */
    let _ = text_indent.insert("indent-36".to_owned(), "text-indent: 9rem;".to_owned()); /* 144px */
    let _ = text_indent.insert("indent-40".to_owned(), "text-indent: 10rem;".to_owned()); /* 160px */
    let _ = text_indent.insert("indent-44".to_owned(), "text-indent: 11rem;".to_owned()); /* 176px */
    let _ = text_indent.insert("indent-48".to_owned(), "text-indent: 12rem;".to_owned()); /* 192px */
    let _ = text_indent.insert("indent-52".to_owned(), "text-indent: 13rem;".to_owned()); /* 208px */
    let _ = text_indent.insert("indent-56".to_owned(), "text-indent: 14rem;".to_owned()); /* 224px */
    let _ = text_indent.insert("indent-60".to_owned(), "text-indent: 15rem;".to_owned()); /* 240px */
    let _ = text_indent.insert("indent-64".to_owned(), "text-indent: 16rem;".to_owned()); /* 256px */
    let _ = text_indent.insert("indent-72".to_owned(), "text-indent: 18rem;".to_owned()); /* 288px */
    let _ = text_indent.insert("indent-80".to_owned(), "text-indent: 20rem;".to_owned()); /* 320px */
    let _ = text_indent.insert("indent-96".to_owned(), "text-indent: 24rem;".to_owned()); /* 384px */

    text_indent
}