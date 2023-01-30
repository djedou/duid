use std::collections::HashMap;


pub(crate) fn max_height() -> HashMap<String, String> {
    let mut max_height = HashMap::new();
    let _ = max_height.insert("max-h-0".to_owned(), "max-height: 0px;".to_owned());
    let _ = max_height.insert("max-h-px".to_owned(), "max-height: 1px;".to_owned());
    let _ = max_height.insert("max-h-0.5".to_owned(), "max-height: 0.125rem;".to_owned()); /* 2px */
    let _ = max_height.insert("max-h-1".to_owned(), "max-height: 0.25rem;".to_owned()); /* 4px */
    let _ = max_height.insert("max-h-1.5".to_owned(), "max-height: 0.375rem;".to_owned()); /* 6px */
    let _ = max_height.insert("max-h-2".to_owned(), "max-height: 0.5rem;".to_owned()); /* 8px */
    let _ = max_height.insert("max-h-2.5".to_owned(), "max-height: 0.625rem;".to_owned()); /* 10px */
    let _ = max_height.insert("max-h-3".to_owned(), "max-height: 0.75rem;".to_owned()); /* 12px */
    let _ = max_height.insert("max-h-3.5".to_owned(), "max-height: 0.875rem;".to_owned()); /* 14px */
    let _ = max_height.insert("max-h-4".to_owned(), "max-height: 1rem;".to_owned()); /* 16px */
    let _ = max_height.insert("max-h-5".to_owned(), "max-height: 1.25rem;".to_owned()); /* 20px */
    let _ = max_height.insert("max-h-6".to_owned(), "max-height: 1.5rem;".to_owned()); /* 24px */
    let _ = max_height.insert("max-h-7".to_owned(), "max-height: 1.75rem;".to_owned()); /* 28px */
    let _ = max_height.insert("max-h-8".to_owned(), "max-height: 2rem;".to_owned()); /* 32px */
    let _ = max_height.insert("max-h-9".to_owned(), "max-height: 2.25rem;".to_owned()); /* 36px */
    let _ = max_height.insert("max-h-10".to_owned(), "max-height: 2.5rem;".to_owned()); /* 40px */
    let _ = max_height.insert("max-h-11".to_owned(), "max-height: 2.75rem;".to_owned()); /* 44px */
    let _ = max_height.insert("max-h-12".to_owned(), "max-height: 3rem;".to_owned()); /* 48px */
    let _ = max_height.insert("max-h-14".to_owned(), "max-height: 3.5rem;".to_owned()); /* 56px */
    let _ = max_height.insert("max-h-16".to_owned(), "max-height: 4rem;".to_owned()); /* 64px */
    let _ = max_height.insert("max-h-20".to_owned(), "max-height: 5rem;".to_owned()); /* 80px */
    let _ = max_height.insert("max-h-24".to_owned(), "max-height: 6rem;".to_owned()); /* 96px */
    let _ = max_height.insert("max-h-28".to_owned(), "max-height: 7rem;".to_owned()); /* 112px */
    let _ = max_height.insert("max-h-32".to_owned(), "max-height: 8rem;".to_owned()); /* 128px */
    let _ = max_height.insert("max-h-36".to_owned(), "max-height: 9rem;".to_owned()); /* 144px */
    let _ = max_height.insert("max-h-40".to_owned(), "max-height: 10rem;".to_owned()); /* 160px */
    let _ = max_height.insert("max-h-44".to_owned(), "max-height: 11rem;".to_owned()); /* 176px */
    let _ = max_height.insert("max-h-48".to_owned(), "max-height: 12rem;".to_owned()); /* 192px */
    let _ = max_height.insert("max-h-52".to_owned(), "max-height: 13rem;".to_owned()); /* 208px */
    let _ = max_height.insert("max-h-56".to_owned(), "max-height: 14rem;".to_owned()); /* 224px */
    let _ = max_height.insert("max-h-60".to_owned(), "max-height: 15rem;".to_owned()); /* 240px */
    let _ = max_height.insert("max-h-64".to_owned(), "max-height: 16rem;".to_owned()); /* 256px */
    let _ = max_height.insert("max-h-72".to_owned(), "max-height: 18rem;".to_owned()); /* 288px */
    let _ = max_height.insert("max-h-80".to_owned(), "max-height: 20rem;".to_owned()); /* 320px */
    let _ = max_height.insert("max-h-96".to_owned(), "max-height: 24rem;".to_owned()); /* 384px */
    let _ = max_height.insert("max-h-full".to_owned(), "max-height: 100%;".to_owned());
    let _ = max_height.insert("max-h-screen".to_owned(), "max-height: 100vh;".to_owned());
    let _ = max_height.insert("max-h-min".to_owned(), "max-height: min-content;".to_owned());
    let _ = max_height.insert("max-h-max".to_owned(), "max-height: max-content;".to_owned());
    let _ = max_height.insert("max-h-fit".to_owned(), "max-height: fit-content;".to_owned());
    let _ = max_height.insert("max-vh-25".to_owned(), "max-height: 25vh;".to_owned());
    let _ = max_height.insert("max-vh-50".to_owned(), "max-height: 50vh;".to_owned());
    let _ = max_height.insert("max-vh-75".to_owned(), "max-height: 75vh;".to_owned());
    let _ = max_height.insert("max-vh-100".to_owned(), "max-height: 100vh;".to_owned());


    max_height
}