use std::collections::HashMap;


pub(crate) fn padding() -> HashMap<String, String> {
    let mut padding = HashMap::new();
    let _ = padding.insert("p-0".to_owned(),	"padding: 0px;".to_owned());
    let _ = padding.insert("px-0".to_owned(), "padding-left: 0px;padding-right: 0px;".to_owned());
    let _ = padding.insert("py-0".to_owned(), "padding-top: 0px;padding-bottom: 0px;".to_owned());
    let _ = padding.insert("pt-0".to_owned(), "padding-top: 0px;".to_owned());
    let _ = padding.insert("pr-0".to_owned(), "padding-right: 0px;".to_owned());
    let _ = padding.insert("pb-0".to_owned(), "padding-bottom: 0px;".to_owned());
    let _ = padding.insert("pl-0".to_owned(), "padding-left: 0px;".to_owned());

    let _ = padding.insert("p-px".to_owned(), "padding: 1px;".to_owned());
    let _ = padding.insert("px-px".to_owned(), "padding-left: 1px;padding-right: 1px;".to_owned());
    let _ = padding.insert("py-px".to_owned(), "padding-top: 1px;padding-bottom: 1px;".to_owned());
    let _ = padding.insert("pt-px".to_owned(), "padding-top: 1px;".to_owned());
    let _ = padding.insert("pr-px".to_owned(), "padding-right: 1px;".to_owned());
    let _ = padding.insert("pb-px".to_owned(), "padding-bottom: 1px;".to_owned());
    let _ = padding.insert("pl-px".to_owned(), "padding-left: 1px;".to_owned());

    let _ = padding.insert("p-0.5".to_owned(), "padding: 0.125rem;".to_owned()); /*0.125rem = 2px */
    let _ = padding.insert("px-0.5".to_owned(), "padding-left: 0.125rem;padding-right: 0.125rem;".to_owned());
    let _ = padding.insert("py-0.5".to_owned(), "padding-top: 0.125rem;padding-bottom: 0.125rem;".to_owned());
    let _ = padding.insert("pt-0.5".to_owned(), "padding-top: 0.125rem;".to_owned());
    let _ = padding.insert("pr-0.5".to_owned(), "padding-right: 0.125rem;".to_owned());
    let _ = padding.insert("pb-0.5".to_owned(), "padding-bottom: 0.125rem;".to_owned());
    let _ = padding.insert("pl-0.5".to_owned(), "padding-left: 0.125rem;".to_owned());

    let _ = padding.insert("p-1".to_owned(),	"padding: 0.25rem;".to_owned()); /*0.25rem = 4px */
    let _ = padding.insert("px-1".to_owned(), "padding-left: 0.25rem;padding-right: 0.25rem;".to_owned());
    let _ = padding.insert("py-1".to_owned(), "padding-top: 0.25rem;padding-bottom: 0.25rem;".to_owned());
    let _ = padding.insert("pt-1".to_owned(), "padding-top: 0.25rem;".to_owned());
    let _ = padding.insert("pr-1".to_owned(), "padding-right: 0.25rem;".to_owned());
    let _ = padding.insert("pb-1".to_owned(), "padding-bottom: 0.25rem;".to_owned());
    let _ = padding.insert("pl-1".to_owned(), "padding-left: 0.25rem;".to_owned());

    let _ = padding.insert("p-1.5".to_owned(), "padding: 0.375rem;".to_owned()); /*0.375rem = 4px */
    let _ = padding.insert("px-1.5".to_owned(), "padding-left: 0.375rem;padding-right: 0.375rem;".to_owned());
    let _ = padding.insert("py-1.5".to_owned(), "padding-top: 0.375rem;padding-bottom: 0.375rem;".to_owned());
    let _ = padding.insert("pt-1.5".to_owned(), "padding-top: 0.375rem;".to_owned());
    let _ = padding.insert("pr-1.5".to_owned(), "padding-right: 0.375rem;".to_owned());
    let _ = padding.insert("pb-1.5".to_owned(), "padding-bottom: 0.375rem;".to_owned());
    let _ = padding.insert("pl-1.5".to_owned(), "padding-left: 0.375rem;".to_owned());

    let _ = padding.insert("p-2".to_owned(), "padding: 0.5rem;".to_owned()); /*0.5rem = 8px */
    let _ = padding.insert("px-2".to_owned(), "padding-left: 0.5rem;padding-right: 0.5rem;".to_owned());
    let _ = padding.insert("py-2".to_owned(), "padding-top: 0.5rem;padding-bottom: 0.5rem;".to_owned());
    let _ = padding.insert("pt-2".to_owned(), "padding-top: 0.5rem;".to_owned());
    let _ = padding.insert("pr-2".to_owned(), "padding-right: 0.5rem;".to_owned());
    let _ = padding.insert("pb-2".to_owned(), "padding-bottom: 0.5rem;".to_owned());
    let _ = padding.insert("pl-2".to_owned(), "padding-left: 0.5rem;".to_owned());

    let _ = padding.insert("p-2".to_owned(), "padding: 0.5rem;".to_owned()); /*0.5rem = 8px */
    let _ = padding.insert("px-2".to_owned(), "padding-left: 0.5rem;padding-right: 0.5rem;".to_owned());
    let _ = padding.insert("py-2".to_owned(), "padding-top: 0.5rem;padding-bottom: 0.5rem;".to_owned());
    let _ = padding.insert("pt-2".to_owned(), "padding-top: 0.5rem;".to_owned());
    let _ = padding.insert("pr-2".to_owned(), "padding-right: 0.5rem;".to_owned());
    let _ = padding.insert("pb-2".to_owned(), "padding-bottom: 0.5rem;".to_owned());
    let _ = padding.insert("pl-2".to_owned(), "padding-left: 0.5rem;".to_owned());

    let _ = padding.insert("p-2.5".to_owned(), "padding: 0.625rem;".to_owned()); /*0.625rem = 10px */
    let _ = padding.insert("px-2.5".to_owned(), "padding-left: 0.625rem;padding-right: 0.625rem;".to_owned());
    let _ = padding.insert("py-2.5".to_owned(), "padding-top: 0.625rem;padding-bottom: 0.625rem;".to_owned());
    let _ = padding.insert("pt-2.5".to_owned(), "padding-top: 0.625rem;".to_owned());
    let _ = padding.insert("pr-2.5".to_owned(), "padding-right: 0.625rem;".to_owned());
    let _ = padding.insert("pb-2.5".to_owned(), "padding-bottom: 0.625rem;".to_owned());
    let _ = padding.insert("pl-2.5".to_owned(), "padding-left: 0.625rem;".to_owned());

    let _ = padding.insert("p-3".to_owned(), "padding: 0.75rem;".to_owned()); /*0.75rem = 12px */
    let _ = padding.insert("px-3".to_owned(), "padding-left: 0.75rem;padding-right: 0.75rem;".to_owned());
    let _ = padding.insert("py-3".to_owned(), "padding-top: 0.75rem;padding-bottom: 0.75rem;".to_owned());
    let _ = padding.insert("pt-3".to_owned(), "padding-top: 0.75rem;".to_owned());
    let _ = padding.insert("pr-3".to_owned(), "padding-right: 0.75rem;".to_owned());
    let _ = padding.insert("pb-3".to_owned(), "padding-bottom: 0.75rem;".to_owned());
    let _ = padding.insert("pl-3".to_owned(), "padding-left: 0.75rem;".to_owned());

    let _ = padding.insert("p-3.5".to_owned(), "padding: 0.875rem;".to_owned()); /*0.875rem = 14px */
    let _ = padding.insert("px-3.5".to_owned(), "padding-left: 0.875rem;padding-right: 0.875rem;".to_owned());
    let _ = padding.insert("py-3.5".to_owned(), "padding-top: 0.875rem;padding-bottom: 0.875rem;".to_owned());
    let _ = padding.insert("pt-3.5".to_owned(), "padding-top: 0.875rem;".to_owned());
    let _ = padding.insert("pr-3.5".to_owned(), "padding-right: 0.875rem;".to_owned());
    let _ = padding.insert("pb-3.5".to_owned(), "padding-bottom: 0.875rem;".to_owned());
    let _ = padding.insert("pl-3.5".to_owned(), "padding-left: 0.875rem;".to_owned());

    let _ = padding.insert("p-4".to_owned(), "padding: 1rem;".to_owned()); /*1rem = 16px */
    let _ = padding.insert("px-4".to_owned(), "padding-left: 1rem;padding-right: 1rem;".to_owned());
    let _ = padding.insert("py-4".to_owned(), "padding-top: 1rem;padding-bottom: 1rem;".to_owned());
    let _ = padding.insert("pt-4".to_owned(), "padding-top: 1rem;".to_owned());
    let _ = padding.insert("pr-4".to_owned(), "padding-right: 1rem;".to_owned());
    let _ = padding.insert("pb-4".to_owned(), "padding-bottom: 1rem;".to_owned());
    let _ = padding.insert("pl-4".to_owned(), "padding-left: 1rem;".to_owned());

    let _ = padding.insert("p-5".to_owned(), "padding: 1.25rem;".to_owned()); /*1.25rem = 20px */
    let _ = padding.insert("px-5".to_owned(), "padding-left: 1.25rem;padding-right: 1.25rem;".to_owned());
    let _ = padding.insert("py-5".to_owned(), "padding-top: 1.25rem;padding-bottom: 1.25rem;".to_owned());
    let _ = padding.insert("pt-5".to_owned(), "padding-top: 1.25rem;".to_owned());
    let _ = padding.insert("pr-5".to_owned(), "padding-right: 1.25rem;".to_owned());
    let _ = padding.insert("pb-5".to_owned(), "padding-bottom: 1.25rem;".to_owned());
    let _ = padding.insert("pl-5".to_owned(), "padding-left: 1.25rem;".to_owned());

    let _ = padding.insert("p-6".to_owned(), "padding: 1.5rem;".to_owned()); /*1.5rem = 24px */
    let _ = padding.insert("px-6".to_owned(), "padding-left: 1.5rem;padding-right: 1.5rem;".to_owned());
    let _ = padding.insert("py-6".to_owned(), "padding-top: 1.5rem;padding-bottom: 1.5rem;".to_owned());
    let _ = padding.insert("pt-6".to_owned(), "padding-top: 1.5rem;".to_owned());
    let _ = padding.insert("pr-6".to_owned(), "padding-right: 1.5rem;".to_owned());
    let _ = padding.insert("pb-6".to_owned(), "padding-bottom: 1.5rem;".to_owned());
    let _ = padding.insert("pl-6".to_owned(), "padding-left: 1.5rem;".to_owned());

    let _ = padding.insert("p-7".to_owned(), "padding: 1.75rem;".to_owned()); /*1.75rem = 28px */
    let _ = padding.insert("px-7".to_owned(), "padding-left: 1.75rem;padding-right: 1.75rem;".to_owned());
    let _ = padding.insert("py-7".to_owned(), "padding-top: 1.75rem;padding-bottom: 1.75rem;".to_owned());
    let _ = padding.insert("pt-7".to_owned(), "padding-top: 1.75rem;".to_owned());
    let _ = padding.insert("pr-7".to_owned(), "padding-right: 1.75rem;".to_owned());
    let _ = padding.insert("pb-7".to_owned(), "padding-bottom: 1.75rem;".to_owned());
    let _ = padding.insert("pl-7".to_owned(), "padding-left: 1.75rem;".to_owned());

    let _ = padding.insert("p-8".to_owned(), "padding: 2rem;".to_owned()); /*2rem = 32px */
    let _ = padding.insert("px-8".to_owned(), "padding-left: 2rem;padding-right: 2rem;".to_owned());
    let _ = padding.insert("py-8".to_owned(), "padding-top: 2rem;padding-bottom: 2rem;".to_owned());
    let _ = padding.insert("pt-8".to_owned(), "padding-top: 2rem;".to_owned());
    let _ = padding.insert("pr-8".to_owned(), "padding-right: 2rem;".to_owned());
    let _ = padding.insert("pb-8".to_owned(), "padding-bottom: 2rem;".to_owned());
    let _ = padding.insert("pl-8".to_owned(), "padding-left: 2rem;".to_owned());

    let _ = padding.insert("p-9".to_owned(), "padding: 2.25rem;".to_owned()); /*2.25rem = 36px */
    let _ = padding.insert("px-9".to_owned(), "padding-left: 2.25rem;padding-right: 2.25rem;".to_owned());
    let _ = padding.insert("py-9".to_owned(), "padding-top: 2.25rem;padding-bottom: 2.25rem;".to_owned());
    let _ = padding.insert("pt-9".to_owned(), "padding-top: 2.25rem;".to_owned());
    let _ = padding.insert("pr-9".to_owned(), "padding-right: 2.25rem;".to_owned());
    let _ = padding.insert("pb-9".to_owned(), "padding-bottom: 2.25rem;".to_owned());
    let _ = padding.insert("pl-9".to_owned(), "padding-left: 2.25rem;".to_owned());

    let _ = padding.insert("p-9".to_owned(), "padding: 2.25rem;".to_owned()); /*2.25rem = 36px */
    let _ = padding.insert("px-9".to_owned(), "padding-left: 2.25rem;padding-right: 2.25rem;".to_owned());
    let _ = padding.insert("py-9".to_owned(), "padding-top: 2.25rem;padding-bottom: 2.25rem;".to_owned());
    let _ = padding.insert("pt-9".to_owned(), "padding-top: 2.25rem;".to_owned());
    let _ = padding.insert("pr-9".to_owned(), "padding-right: 2.25rem;".to_owned());
    let _ = padding.insert("pb-9".to_owned(), "padding-bottom: 2.25rem;".to_owned());
    let _ = padding.insert("pl-9".to_owned(), "padding-left: 2.25rem;".to_owned());

    let _ = padding.insert("p-10".to_owned(), "padding: 2.5rem;".to_owned()); /*2.5rem = 40px */
    let _ = padding.insert("px-10".to_owned(), "padding-left: 2.5rem;padding-right: 2.5rem;".to_owned());
    let _ = padding.insert("py-10".to_owned(), "padding-top: 2.5rem;padding-bottom: 2.5rem;".to_owned());
    let _ = padding.insert("pt-10".to_owned(), "padding-top: 2.5rem;".to_owned());
    let _ = padding.insert("pr-10".to_owned(), "padding-right: 2.5rem;".to_owned());
    let _ = padding.insert("pb-10".to_owned(), "padding-bottom: 2.5rem;".to_owned());
    let _ = padding.insert("pl-10".to_owned(), "padding-left: 2.5rem;".to_owned());

    let _ = padding.insert("p-11".to_owned(), "padding: 2.75rem;".to_owned()); /*2.75rem = 44px */
    let _ = padding.insert("px-11".to_owned(), "padding-left: 2.75rem;padding-right: 2.75rem;".to_owned());
    let _ = padding.insert("py-11".to_owned(), "padding-top: 2.75rem;padding-bottom: 2.75rem;".to_owned());
    let _ = padding.insert("pt-11".to_owned(), "padding-top: 2.75rem;".to_owned());
    let _ = padding.insert("pr-11".to_owned(), "padding-right: 2.75rem;".to_owned());
    let _ = padding.insert("pb-11".to_owned(), "padding-bottom: 2.75rem;".to_owned());
    let _ = padding.insert("pl-11".to_owned(), "padding-left: 2.75rem;".to_owned());

    let _ = padding.insert("p-12".to_owned(), "padding: 3rem;".to_owned()); /*3rem = 48px */
    let _ = padding.insert("px-12".to_owned(), "padding-left: 3rem;padding-right: 3rem;".to_owned());
    let _ = padding.insert("py-12".to_owned(), "padding-top: 3rem;padding-bottom: 3rem;".to_owned());
    let _ = padding.insert("pt-12".to_owned(), "padding-top: 3rem;".to_owned());
    let _ = padding.insert("pr-12".to_owned(), "padding-right: 3rem;".to_owned());
    let _ = padding.insert("pb-12".to_owned(), "padding-bottom: 3rem;".to_owned());
    let _ = padding.insert("pl-12".to_owned(), "padding-left: 3rem;".to_owned());

    let _ = padding.insert("p-13".to_owned(), "padding: 3.25rem;".to_owned()); /*3.25rem = 52px */
    let _ = padding.insert("px-13".to_owned(), "padding-left: 3.25rem;padding-right: 3.25rem;".to_owned());
    let _ = padding.insert("py-13".to_owned(), "padding-top: 3.25rem;padding-bottom: 3.25rem;".to_owned());
    let _ = padding.insert("pt-13".to_owned(), "padding-top: 3.25rem;".to_owned());
    let _ = padding.insert("pr-13".to_owned(), "padding-right: 3.25rem;".to_owned());
    let _ = padding.insert("pb-13".to_owned(), "padding-bottom: 3.25rem;".to_owned());
    let _ = padding.insert("pl-13".to_owned(), "padding-left: 3.25rem;".to_owned());

    let _ = padding.insert("p-14".to_owned(), "padding: 3.5rem;".to_owned()); /*3.5rem = 56px */
    let _ = padding.insert("px-14".to_owned(), "padding-left: 3.5rem;padding-right: 3.5rem;".to_owned());
    let _ = padding.insert("py-14".to_owned(), "padding-top: 3.5rem;padding-bottom: 3.5rem;".to_owned());
    let _ = padding.insert("pt-14".to_owned(), "padding-top: 3.5rem;".to_owned());
    let _ = padding.insert("pr-14".to_owned(), "padding-right: 3.5rem;".to_owned());
    let _ = padding.insert("pb-14".to_owned(), "padding-bottom: 3.5rem;".to_owned());
    let _ = padding.insert("pl-14".to_owned(), "padding-left: 3.5rem;".to_owned());

    let _ = padding.insert("p-15".to_owned(), "padding: 3.75rem;".to_owned()); /*3.75rem = 60px */
    let _ = padding.insert("px-15".to_owned(), "padding-left: 3.75rem;padding-right: 3.75rem;".to_owned());
    let _ = padding.insert("py-15".to_owned(), "padding-top: 3.75rem;padding-bottom: 3.75rem;".to_owned());
    let _ = padding.insert("pt-15".to_owned(), "padding-top: 3.75rem;".to_owned());
    let _ = padding.insert("pr-15".to_owned(), "padding-right: 3.75rem;".to_owned());
    let _ = padding.insert("pb-15".to_owned(), "padding-bottom: 3.75rem;".to_owned());
    let _ = padding.insert("pl-15".to_owned(), "padding-left: 3.75rem;".to_owned());

    let _ = padding.insert("p-16".to_owned(), "padding: 4rem;".to_owned()); /*4rem = 64px */
    let _ = padding.insert("px-16".to_owned(), "padding-left: 4rem;padding-right: 4rem;".to_owned());
    let _ = padding.insert("py-16".to_owned(), "padding-top: 4rem;padding-bottom: 4rem;".to_owned());
    let _ = padding.insert("pt-16".to_owned(), "padding-top: 4rem;".to_owned());
    let _ = padding.insert("pr-16".to_owned(), "padding-right: 4rem;".to_owned());
    let _ = padding.insert("pb-16".to_owned(), "padding-bottom: 4rem;".to_owned());
    let _ = padding.insert("pl-16".to_owned(), "padding-left: 4rem;".to_owned());

    let _ = padding.insert("p-20".to_owned(), "padding: 5rem;".to_owned()); /*5rem = 80px */
    let _ = padding.insert("px-20".to_owned(), "padding-left: 5rem;padding-right: 5rem;".to_owned());
    let _ = padding.insert("py-20".to_owned(), "padding-top: 5rem;padding-bottom: 5rem;".to_owned());
    let _ = padding.insert("pt-20".to_owned(), "padding-top: 5rem;".to_owned());
    let _ = padding.insert("pr-20".to_owned(), "padding-right: 5rem;".to_owned());
    let _ = padding.insert("pb-20".to_owned(), "padding-bottom: 5rem;".to_owned());
    let _ = padding.insert("pl-20".to_owned(), "padding-left: 5rem;".to_owned());

    let _ = padding.insert("p-24".to_owned(), "padding: 6rem;".to_owned()); /*6rem = 80px */
    let _ = padding.insert("px-24".to_owned(), "padding-left: 6rem;padding-right: 6rem;".to_owned());
    let _ = padding.insert("py-24".to_owned(), "padding-top: 6rem;padding-bottom: 6rem;".to_owned());
    let _ = padding.insert("pt-24".to_owned(), "padding-top: 6rem;".to_owned());
    let _ = padding.insert("pr-24".to_owned(), "padding-right: 6rem;".to_owned());
    let _ = padding.insert("pb-24".to_owned(), "padding-bottom: 6rem;".to_owned());
    let _ = padding.insert("pl-24".to_owned(), "padding-left: 6rem;".to_owned());

    let _ = padding.insert("p-28".to_owned(), "padding: 7rem;".to_owned()); /*7rem = 112px */
    let _ = padding.insert("px-28".to_owned(), "padding-left: 7rem;padding-right: 7rem;".to_owned());
    let _ = padding.insert("py-28".to_owned(), "padding-top: 7rem;padding-bottom: 7rem;".to_owned());
    let _ = padding.insert("pt-28".to_owned(), "padding-top: 7rem;".to_owned());
    let _ = padding.insert("pr-28".to_owned(), "padding-right: 7rem;".to_owned());
    let _ = padding.insert("pb-28".to_owned(), "padding-bottom: 7rem;".to_owned());
    let _ = padding.insert("pl-28".to_owned(), "padding-left: 7rem;".to_owned());

    let _ = padding.insert("p-32".to_owned(), "padding: 8rem;".to_owned()); /*8rem = 128px */
    let _ = padding.insert("px-32".to_owned(), "padding-left: 8rem;padding-right: 8rem;".to_owned());
    let _ = padding.insert("py-32".to_owned(), "padding-top: 8rem;padding-bottom: 8rem;".to_owned());
    let _ = padding.insert("pt-32".to_owned(), "padding-top: 8rem;".to_owned());
    let _ = padding.insert("pr-32".to_owned(), "padding-right: 8rem;".to_owned());
    let _ = padding.insert("pb-32".to_owned(), "padding-bottom: 8rem;".to_owned());
    let _ = padding.insert("pl-32".to_owned(), "padding-left: 8rem;".to_owned());

    let _ = padding.insert("p-36".to_owned(), "padding: 9rem;".to_owned()); /*9rem = 144px */
    let _ = padding.insert("px-36".to_owned(), "padding-left: 9rem;padding-right: 9rem;".to_owned());
    let _ = padding.insert("py-36".to_owned(), "padding-top: 9rem;padding-bottom: 9rem;".to_owned());
    let _ = padding.insert("pt-36".to_owned(), "padding-top: 9rem;".to_owned());
    let _ = padding.insert("pr-36".to_owned(), "padding-right: 9rem;".to_owned());
    let _ = padding.insert("pb-36".to_owned(), "padding-bottom: 9rem;".to_owned());
    let _ = padding.insert("pl-36".to_owned(), "padding-left: 9rem;".to_owned());

    let _ = padding.insert("p-40".to_owned(), "padding: 10rem;".to_owned()); /*10rem = 144px */
    let _ = padding.insert("px-40".to_owned(), "padding-left: 10rem;padding-right: 10rem;".to_owned());
    let _ = padding.insert("py-40".to_owned(), "padding-top: 10rem;padding-bottom: 10rem;".to_owned());
    let _ = padding.insert("pt-40".to_owned(), "padding-top: 10rem;".to_owned());
    let _ = padding.insert("pr-40".to_owned(), "padding-right: 10rem;".to_owned());
    let _ = padding.insert("pb-40".to_owned(), "padding-bottom: 10rem;".to_owned());
    let _ = padding.insert("pl-40".to_owned(), "padding-left: 10rem;".to_owned());

    let _ = padding.insert("p-44".to_owned(), "padding: 11rem;".to_owned()); /*11rem = 176px */
    let _ = padding.insert("px-44".to_owned(), "padding-left: 11rem;padding-right: 11rem;".to_owned());
    let _ = padding.insert("py-44".to_owned(), "padding-top: 11rem;padding-bottom: 11rem;".to_owned());
    let _ = padding.insert("pt-44".to_owned(), "padding-top: 11rem;".to_owned());
    let _ = padding.insert("pr-44".to_owned(), "padding-right: 11rem;".to_owned());
    let _ = padding.insert("pb-44".to_owned(), "padding-bottom: 11rem;".to_owned());
    let _ = padding.insert("pl-44".to_owned(), "padding-left: 11rem;".to_owned());

    let _ = padding.insert("p-48".to_owned(), "padding: 12rem;".to_owned()); /*12rem = 176px */
    let _ = padding.insert("px-48".to_owned(), "padding-left: 12rem;padding-right: 12rem;".to_owned());
    let _ = padding.insert("py-48".to_owned(), "padding-top: 12rem;padding-bottom: 12rem;".to_owned());
    let _ = padding.insert("pt-48".to_owned(), "padding-top: 12rem;".to_owned());
    let _ = padding.insert("pr-48".to_owned(), "padding-right: 12rem;".to_owned());
    let _ = padding.insert("pb-48".to_owned(), "padding-bottom: 12rem;".to_owned());
    let _ = padding.insert("pl-48".to_owned(), "padding-left: 12rem;".to_owned());

    let _ = padding.insert("p-52".to_owned(), "padding: 13rem;".to_owned()); /*13rem = 208px */
    let _ = padding.insert("px-52".to_owned(), "padding-left: 13rem;padding-right: 13rem;".to_owned());
    let _ = padding.insert("py-52".to_owned(), "padding-top: 13rem;padding-bottom: 13rem;".to_owned());
    let _ = padding.insert("pt-52".to_owned(), "padding-top: 13rem;".to_owned());
    let _ = padding.insert("pr-52".to_owned(), "padding-right: 13rem;".to_owned());
    let _ = padding.insert("pb-52".to_owned(), "padding-bottom: 13rem;".to_owned());
    let _ = padding.insert("pl-52".to_owned(), "padding-left: 13rem;".to_owned());

    let _ = padding.insert("p-56".to_owned(), "padding: 14rem;".to_owned()); /*14rem = 224px */
    let _ = padding.insert("px-56".to_owned(), "padding-left: 14rem;padding-right: 14rem;".to_owned());
    let _ = padding.insert("py-56".to_owned(), "padding-top: 14rem;padding-bottom: 14rem;".to_owned());
    let _ = padding.insert("pt-56".to_owned(), "padding-top: 14rem;".to_owned());
    let _ = padding.insert("pr-56".to_owned(), "padding-right: 14rem;".to_owned());
    let _ = padding.insert("pb-56".to_owned(), "padding-bottom: 14rem;".to_owned());
    let _ = padding.insert("pl-56".to_owned(), "padding-left: 14rem;".to_owned());

    let _ = padding.insert("p-60".to_owned(), "padding: 15rem;".to_owned()); /*15rem = 224px */
    let _ = padding.insert("px-60".to_owned(), "padding-left: 15rem;padding-right: 15rem;".to_owned());
    let _ = padding.insert("py-60".to_owned(), "padding-top: 15rem;padding-bottom: 15rem;".to_owned());
    let _ = padding.insert("pt-60".to_owned(), "padding-top: 15rem;".to_owned());
    let _ = padding.insert("pr-60".to_owned(), "padding-right: 15rem;".to_owned());
    let _ = padding.insert("pb-60".to_owned(), "padding-bottom: 15rem;".to_owned());
    let _ = padding.insert("pl-60".to_owned(), "padding-left: 15rem;".to_owned());

    let _ = padding.insert("p-64".to_owned(), "padding: 16rem;".to_owned()); /*16rem = 256px */
    let _ = padding.insert("px-64".to_owned(), "padding-left: 16rem;padding-right: 16rem;".to_owned());
    let _ = padding.insert("py-64".to_owned(), "padding-top: 16rem;padding-bottom: 16rem;".to_owned());
    let _ = padding.insert("pt-64".to_owned(), "padding-top: 16rem;".to_owned());
    let _ = padding.insert("pr-64".to_owned(), "padding-right: 16rem;".to_owned());
    let _ = padding.insert("pb-64".to_owned(), "padding-bottom: 16rem;".to_owned());
    let _ = padding.insert("pl-64".to_owned(), "padding-left: 16rem;".to_owned());

    let _ = padding.insert("p-72".to_owned(), "padding: 18rem;".to_owned()); /*18rem = 288px */
    let _ = padding.insert("px-72".to_owned(), "padding-left: 18rem;padding-right: 18rem;".to_owned());
    let _ = padding.insert("py-72".to_owned(), "padding-top: 18rem;padding-bottom: 18rem;".to_owned());
    let _ = padding.insert("pt-72".to_owned(), "padding-top: 18rem;".to_owned());
    let _ = padding.insert("pr-72".to_owned(), "padding-right: 18rem;".to_owned());
    let _ = padding.insert("pb-72".to_owned(), "padding-bottom: 18rem;".to_owned());
    let _ = padding.insert("pl-72".to_owned(), "padding-left: 18rem;".to_owned());

    let _ = padding.insert("p-80".to_owned(), "padding: 20rem;".to_owned()); /*20rem = 320px */
    let _ = padding.insert("px-80".to_owned(), "padding-left: 20rem;padding-right: 20rem;".to_owned());
    let _ = padding.insert("py-80".to_owned(), "padding-top: 20rem;padding-bottom: 20rem;".to_owned());
    let _ = padding.insert("pt-80".to_owned(), "padding-top: 20rem;".to_owned());
    let _ = padding.insert("pr-80".to_owned(), "padding-right: 20rem;".to_owned());
    let _ = padding.insert("pb-80".to_owned(), "padding-bottom: 20rem;".to_owned());
    let _ = padding.insert("pl-80".to_owned(), "padding-left: 20rem;".to_owned());

    let _ = padding.insert("p-96".to_owned(), "padding: 24rem;".to_owned()); /*24rem = 384px */
    let _ = padding.insert("px-96".to_owned(), "padding-left: 24rem;padding-right: 24rem;".to_owned());
    let _ = padding.insert("py-96".to_owned(), "padding-top: 24rem;padding-bottom: 24rem;".to_owned());
    let _ = padding.insert("pt-96".to_owned(), "padding-top: 24rem;".to_owned());
    let _ = padding.insert("pr-96".to_owned(), "padding-right: 24rem;".to_owned());
    let _ = padding.insert("pb-96".to_owned(), "padding-bottom: 24rem;".to_owned());
    let _ = padding.insert("pl-96".to_owned(), "padding-left: 24rem;".to_owned());

    let _ = padding.insert("p-auto".to_owned(), "padding: auto;".to_owned());
    let _ = padding.insert("px-auto".to_owned(), "padding-left: auto;padding-right: auto;".to_owned());
    let _ = padding.insert("py-auto".to_owned(), "padding-top: auto;padding-bottom: auto;".to_owned());
    let _ = padding.insert("pt-auto".to_owned(), "padding-top: auto;".to_owned());
    let _ = padding.insert("pr-auto".to_owned(), "padding-right: auto;".to_owned());
    let _ = padding.insert("pb-auto".to_owned(), "padding-bottom: auto;".to_owned());
    let _ = padding.insert("pl-auto".to_owned(), "padding-left: auto;".to_owned());
    
    padding
}