use std::collections::HashMap;


pub(crate) fn scroll_padding() -> HashMap<String, String> {
    let mut scroll_padding = HashMap::new();
    let _ = scroll_padding.insert("scroll-p-0".to_owned(), "scroll-padding: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-px-0".to_owned(), "scroll-padding-left: 0px;scroll-padding-right: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-py-0".to_owned(), "scroll-padding-top: 0px;scroll-padding-bottom: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-0".to_owned(), "scroll-padding-top: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-0".to_owned(), "scroll-padding-right: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-0".to_owned(), "scroll-padding-bottom: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-0".to_owned(), "scroll-padding-left: 0px;".to_owned());
    let _ = scroll_padding.insert("scroll-p-px".to_owned(), "scroll-padding: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-px-px".to_owned(), "scroll-padding-left: 1px;scroll-padding-right: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-py-px".to_owned(), "scroll-padding-top: 1px;scroll-padding-bottom: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-px".to_owned(), "scroll-padding-top: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-px".to_owned(), "scroll-padding-right: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-px".to_owned(), "scroll-padding-bottom: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-px".to_owned(), "scroll-padding-left: 1px;".to_owned());
    let _ = scroll_padding.insert("scroll-p-0.5".to_owned(), "scroll-padding: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-0.5".to_owned(), "scroll-padding-left: 0.125rem;scroll-padding-right: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-0.5".to_owned(), "scroll-padding-top: 0.125rem;scroll-padding-bottom: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-0.5".to_owned(), "scroll-padding-top: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-0.5".to_owned(), "scroll-padding-right: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-0.5".to_owned(), "scroll-padding-bottom: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-0.5".to_owned(), "scroll-padding-left: 0.125rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-1".to_owned(), "scroll-padding: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-1".to_owned(), "scroll-padding-left: 0.25rem;scroll-padding-right: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-1".to_owned(), "scroll-padding-top: 0.25rem;scroll-padding-bottom: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-1".to_owned(), "scroll-padding-top: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-1".to_owned(), "scroll-padding-right: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-1".to_owned(), "scroll-padding-bottom: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-1".to_owned(), "scroll-padding-left: 0.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-1.5".to_owned(), "scroll-padding: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-1.5".to_owned(), "scroll-padding-left: 0.375rem;scroll-padding-right: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-1.5".to_owned(), "scroll-padding-top: 0.375rem;scroll-padding-bottom: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-1.5".to_owned(), "scroll-padding-top: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-1.5".to_owned(), "scroll-padding-right: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-1.5".to_owned(), "scroll-padding-bottom: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-1.5".to_owned(), "scroll-padding-left: 0.375rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-2".to_owned(), "scroll-padding: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-2".to_owned(), "scroll-padding-left: 0.5rem;scroll-padding-right: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-2".to_owned(), "scroll-padding-top: 0.5rem;scroll-padding-bottom: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-2".to_owned(), "scroll-padding-top: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-2".to_owned(), "scroll-padding-right: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-2".to_owned(), "scroll-padding-bottom: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-2".to_owned(), "scroll-padding-left: 0.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-2.5".to_owned(), "scroll-padding: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-2.5".to_owned(), "scroll-padding-left: 0.625rem;scroll-padding-right: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-2.5".to_owned(), "scroll-padding-top: 0.625rem;scroll-padding-bottom: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-2.5".to_owned(), "scroll-padding-top: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-2.5".to_owned(), "scroll-padding-right: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-2.5".to_owned(), "scroll-padding-bottom: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-2.5".to_owned(), "scroll-padding-left: 0.625rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-3".to_owned(), "scroll-padding: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-3".to_owned(), "scroll-padding-left: 0.75rem;scroll-padding-right: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-3".to_owned(), "scroll-padding-top: 0.75rem;scroll-padding-bottom: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-3".to_owned(), "scroll-padding-top: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-3".to_owned(), "scroll-padding-right: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-3".to_owned(), "scroll-padding-bottom: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-3".to_owned(), "scroll-padding-left: 0.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-3.5".to_owned(), "scroll-padding: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-3.5".to_owned(), "scroll-padding-left: 0.875rem;scroll-padding-right: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-3.5".to_owned(), "scroll-padding-top: 0.875rem;scroll-padding-bottom: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-3.5".to_owned(), "scroll-padding-top: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-3.5".to_owned(), "scroll-padding-right: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-3.5".to_owned(), "scroll-padding-bottom: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-3.5".to_owned(), "scroll-padding-left: 0.875rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-4".to_owned(), "scroll-padding: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-4".to_owned(), "scroll-padding-left: 1rem;scroll-padding-right: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-4".to_owned(), "scroll-padding-top: 1rem;scroll-padding-bottom: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-4".to_owned(), "scroll-padding-top: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-4".to_owned(), "scroll-padding-right: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-4".to_owned(), "scroll-padding-bottom: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-4".to_owned(), "scroll-padding-left: 1rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-5".to_owned(), "scroll-padding: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-5".to_owned(), "scroll-padding-left: 1.25rem;scroll-padding-right: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-5".to_owned(), "scroll-padding-top: 1.25rem;scroll-padding-bottom: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-5".to_owned(), "scroll-padding-top: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-5".to_owned(), "scroll-padding-right: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-5".to_owned(), "scroll-padding-bottom: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-5".to_owned(), "scroll-padding-left: 1.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-6".to_owned(), "scroll-padding: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-6".to_owned(), "scroll-padding-left: 1.5rem;scroll-padding-right: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-6".to_owned(), "scroll-padding-top: 1.5rem;scroll-padding-bottom: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-6".to_owned(), "scroll-padding-top: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-6".to_owned(), "scroll-padding-right: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-6".to_owned(), "scroll-padding-bottom: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-6".to_owned(), "scroll-padding-left: 1.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-7".to_owned(), "scroll-padding: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-7".to_owned(), "scroll-padding-left: 1.75rem;scroll-padding-right: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-7".to_owned(), "scroll-padding-top: 1.75rem;scroll-padding-bottom: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-7".to_owned(), "scroll-padding-top: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-7".to_owned(), "scroll-padding-right: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-7".to_owned(), "scroll-padding-bottom: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-7".to_owned(), "scroll-padding-left: 1.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-8".to_owned(), "scroll-padding: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-8".to_owned(), "scroll-padding-left: 2rem;scroll-padding-right: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-8".to_owned(), "scroll-padding-top: 2rem;scroll-padding-bottom: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-8".to_owned(), "scroll-padding-top: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-8".to_owned(), "scroll-padding-right: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-8".to_owned(), "scroll-padding-bottom: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-8".to_owned(), "scroll-padding-left: 2rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-9".to_owned(), "scroll-padding: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-9".to_owned(), "scroll-padding-left: 2.25rem;scroll-padding-right: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-9".to_owned(), "scroll-padding-top: 2.25rem;scroll-padding-bottom: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-9".to_owned(), "scroll-padding-top: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-9".to_owned(), "scroll-padding-right: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-9".to_owned(), "scroll-padding-bottom: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-9".to_owned(), "scroll-padding-left: 2.25rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-10".to_owned(), "scroll-padding: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-10".to_owned(), "scroll-padding-left: 2.5rem;scroll-padding-right: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-10".to_owned(), "scroll-padding-top: 2.5rem;scroll-padding-bottom: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-10".to_owned(), "scroll-padding-top: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-10".to_owned(), "scroll-padding-right: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-10".to_owned(), "scroll-padding-bottom: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-10".to_owned(), "scroll-padding-left: 2.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-11".to_owned(), "scroll-padding: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-11".to_owned(), "scroll-padding-left: 2.75rem;scroll-padding-right: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-11".to_owned(), "scroll-padding-top: 2.75rem;scroll-padding-bottom: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-11".to_owned(), "scroll-padding-top: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-11".to_owned(), "scroll-padding-right: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-11".to_owned(), "scroll-padding-bottom: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-11".to_owned(), "scroll-padding-left: 2.75rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-12".to_owned(), "scroll-padding: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-12".to_owned(), "scroll-padding-left: 3rem;scroll-padding-right: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-12".to_owned(), "scroll-padding-top: 3rem;scroll-padding-bottom: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-12".to_owned(), "scroll-padding-top: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-12".to_owned(), "scroll-padding-right: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-12".to_owned(), "scroll-padding-bottom: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-12".to_owned(), "scroll-padding-left: 3rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-14".to_owned(), "scroll-padding: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-14".to_owned(), "scroll-padding-left: 3.5rem;scroll-padding-right: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-14".to_owned(), "scroll-padding-top: 3.5rem;scroll-padding-bottom: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-14".to_owned(), "scroll-padding-top: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-14".to_owned(), "scroll-padding-right: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-14".to_owned(), "scroll-padding-bottom: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-14".to_owned(), "scroll-padding-left: 3.5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-16".to_owned(), "scroll-padding: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-16".to_owned(), "scroll-padding-left: 4rem;scroll-padding-right: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-16".to_owned(), "scroll-padding-top: 4rem;scroll-padding-bottom: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-16".to_owned(), "scroll-padding-top: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-16".to_owned(), "scroll-padding-right: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-16".to_owned(), "scroll-padding-bottom: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-16".to_owned(), "scroll-padding-left: 4rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-20".to_owned(), "scroll-padding: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-20".to_owned(), "scroll-padding-left: 5rem;scroll-padding-right: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-20".to_owned(), "scroll-padding-top: 5rem;scroll-padding-bottom: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-20".to_owned(), "scroll-padding-top: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-20".to_owned(), "scroll-padding-right: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-20".to_owned(), "scroll-padding-bottom: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-20".to_owned(), "scroll-padding-left: 5rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-24".to_owned(), "scroll-padding: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-24".to_owned(), "scroll-padding-left: 6rem;scroll-padding-right: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-24".to_owned(), "scroll-padding-top: 6rem;scroll-padding-bottom: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-24".to_owned(), "scroll-padding-top: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-24".to_owned(), "scroll-padding-right: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-24".to_owned(), "scroll-padding-bottom: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-24".to_owned(), "scroll-padding-left: 6rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-28".to_owned(), "scroll-padding: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-28".to_owned(), "scroll-padding-left: 7rem;scroll-padding-right: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-28".to_owned(), "scroll-padding-top: 7rem;scroll-padding-bottom: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-28".to_owned(), "scroll-padding-top: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-28".to_owned(), "scroll-padding-right: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-28".to_owned(), "scroll-padding-bottom: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-28".to_owned(), "scroll-padding-left: 7rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-32".to_owned(), "scroll-padding: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-32".to_owned(), "scroll-padding-left: 8rem;scroll-padding-right: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-32".to_owned(), "scroll-padding-top: 8rem;scroll-padding-bottom: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-32".to_owned(), "scroll-padding-top: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-32".to_owned(), "scroll-padding-right: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-32".to_owned(), "scroll-padding-bottom: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-32".to_owned(), "scroll-padding-left: 8rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-36".to_owned(), "scroll-padding: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-36".to_owned(), "scroll-padding-left: 9rem;scroll-padding-right: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-36".to_owned(), "scroll-padding-top: 9rem;scroll-padding-bottom: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-36".to_owned(), "scroll-padding-top: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-36".to_owned(), "scroll-padding-right: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-36".to_owned(), "scroll-padding-bottom: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-36".to_owned(), "scroll-padding-left: 9rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-40".to_owned(), "scroll-padding: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-40".to_owned(), "scroll-padding-left: 10rem;scroll-padding-right: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-40".to_owned(), "scroll-padding-top: 10rem;scroll-padding-bottom: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-40".to_owned(), "scroll-padding-top: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-40".to_owned(), "scroll-padding-right: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-40".to_owned(), "scroll-padding-bottom: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-40".to_owned(), "scroll-padding-left: 10rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-44".to_owned(), "scroll-padding: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-44".to_owned(), "scroll-padding-left: 11rem;scroll-padding-right: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-44".to_owned(), "scroll-padding-top: 11rem;scroll-padding-bottom: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-44".to_owned(), "scroll-padding-top: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-44".to_owned(), "scroll-padding-right: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-44".to_owned(), "scroll-padding-bottom: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-44".to_owned(), "scroll-padding-left: 11rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-48".to_owned(), "scroll-padding: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-48".to_owned(), "scroll-padding-left: 12rem;scroll-padding-right: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-48".to_owned(), "scroll-padding-top: 12rem;scroll-padding-bottom: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-48".to_owned(), "scroll-padding-top: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-48".to_owned(), "scroll-padding-right: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-48".to_owned(), "scroll-padding-bottom: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-48".to_owned(), "scroll-padding-left: 12rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-52".to_owned(), "scroll-padding: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-52".to_owned(), "scroll-padding-left: 13rem;scroll-padding-right: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-52".to_owned(), "scroll-padding-top: 13rem;scroll-padding-bottom: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-52".to_owned(), "scroll-padding-top: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-52".to_owned(), "scroll-padding-right: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-52".to_owned(), "scroll-padding-bottom: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-52".to_owned(), "scroll-padding-left: 13rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-56".to_owned(), "scroll-padding: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-56".to_owned(), "scroll-padding-left: 14rem;scroll-padding-right: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-56".to_owned(), "scroll-padding-top: 14rem;scroll-padding-bottom: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-56".to_owned(), "scroll-padding-top: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-56".to_owned(), "scroll-padding-right: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-56".to_owned(), "scroll-padding-bottom: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-56".to_owned(), "scroll-padding-left: 14rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-60".to_owned(), "scroll-padding: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-60".to_owned(), "scroll-padding-left: 15rem;scroll-padding-right: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-60".to_owned(), "scroll-padding-top: 15rem;scroll-padding-bottom: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-60".to_owned(), "scroll-padding-top: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-60".to_owned(), "scroll-padding-right: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-60".to_owned(), "scroll-padding-bottom: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-60".to_owned(), "scroll-padding-left: 15rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-64".to_owned(), "scroll-padding: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-64".to_owned(), "scroll-padding-left: 16rem;scroll-padding-right: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-64".to_owned(), "scroll-padding-top: 16rem;scroll-padding-bottom: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-64".to_owned(), "scroll-padding-top: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-64".to_owned(), "scroll-padding-right: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-64".to_owned(), "scroll-padding-bottom: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-64".to_owned(), "scroll-padding-left: 16rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-72".to_owned(), "scroll-padding: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-72".to_owned(), "scroll-padding-left: 18rem;scroll-padding-right: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-72".to_owned(), "scroll-padding-top: 18rem;scroll-padding-bottom: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-72".to_owned(), "scroll-padding-top: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-72".to_owned(), "scroll-padding-right: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-72".to_owned(), "scroll-padding-bottom: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-72".to_owned(), "scroll-padding-left: 18rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-80".to_owned(), "scroll-padding: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-80".to_owned(), "scroll-padding-left: 20rem;scroll-padding-right: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-80".to_owned(), "scroll-padding-top: 20rem;scroll-padding-bottom: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-80".to_owned(), "scroll-padding-top: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-80".to_owned(), "scroll-padding-right: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-80".to_owned(), "scroll-padding-bottom: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-80".to_owned(), "scroll-padding-left: 20rem;".to_owned());
    let _ = scroll_padding.insert("scroll-p-96".to_owned(), "scroll-padding: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-px-96".to_owned(), "scroll-padding-left: 24rem;scroll-padding-right: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-py-96".to_owned(), "scroll-padding-top: 24rem;scroll-padding-bottom: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pt-96".to_owned(), "scroll-padding-top: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pr-96".to_owned(), "scroll-padding-right: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pb-96".to_owned(), "scroll-padding-bottom: 24rem;".to_owned());
    let _ = scroll_padding.insert("scroll-pl-96".to_owned(), "scroll-padding-left: 24rem;".to_owned());
    
    
    scroll_padding
}