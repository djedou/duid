use std::collections::HashMap;


pub(crate) fn margin() -> HashMap<String, String> {
    let mut margin = HashMap::new();
    let _ = margin.insert("m-0".to_owned(),	"margin: 0px;".to_owned());
    let _ = margin.insert("mx-0".to_owned(), "margin-left: 0px;margin-right: 0px;".to_owned());
    let _ = margin.insert("my-0".to_owned(), "margin-top: 0px;margin-bottom: 0px;".to_owned());
    let _ = margin.insert("mt-0".to_owned(), "margin-top: 0px;".to_owned());
    let _ = margin.insert("mr-0".to_owned(), "margin-right: 0px;".to_owned());
    let _ = margin.insert("mb-0".to_owned(), "margin-bottom: 0px;".to_owned());
    let _ = margin.insert("ml-0".to_owned(), "margin-left: 0px;".to_owned());

    let _ = margin.insert("m-px".to_owned(), "margin: 1px;".to_owned());
    let _ = margin.insert("mx-px".to_owned(), "margin-left: 1px;margin-right: 1px;".to_owned());
    let _ = margin.insert("my-px".to_owned(), "margin-top: 1px;margin-bottom: 1px;".to_owned());
    let _ = margin.insert("mt-px".to_owned(), "margin-top: 1px;".to_owned());
    let _ = margin.insert("mr-px".to_owned(), "margin-right: 1px;".to_owned());
    let _ = margin.insert("mb-px".to_owned(), "margin-bottom: 1px;".to_owned());
    let _ = margin.insert("ml-px".to_owned(), "margin-left: 1px;".to_owned());

    let _ = margin.insert("m-0.5".to_owned(), "margin: 0.125rem;".to_owned()); /*0.125rem = 2px */
    let _ = margin.insert("mx-0.5".to_owned(), "margin-left: 0.125rem;margin-right: 0.125rem;".to_owned());
    let _ = margin.insert("my-0.5".to_owned(), "margin-top: 0.125rem;margin-bottom: 0.125rem;".to_owned());
    let _ = margin.insert("mt-0.5".to_owned(), "margin-top: 0.125rem;".to_owned());
    let _ = margin.insert("mr-0.5".to_owned(), "margin-right: 0.125rem;".to_owned());
    let _ = margin.insert("mb-0.5".to_owned(), "margin-bottom: 0.125rem;".to_owned());
    let _ = margin.insert("ml-0.5".to_owned(), "margin-left: 0.125rem;".to_owned());

    let _ = margin.insert("m-1".to_owned(),	"margin: 0.25rem;".to_owned()); /*0.25rem = 4px */
    let _ = margin.insert("mx-1".to_owned(), "margin-left: 0.25rem;margin-right: 0.25rem;".to_owned());
    let _ = margin.insert("my-1".to_owned(), "margin-top: 0.25rem;margin-bottom: 0.25rem;".to_owned());
    let _ = margin.insert("mt-1".to_owned(), "margin-top: 0.25rem;".to_owned());
    let _ = margin.insert("mr-1".to_owned(), "margin-right: 0.25rem;".to_owned());
    let _ = margin.insert("mb-1".to_owned(), "margin-bottom: 0.25rem;".to_owned());
    let _ = margin.insert("ml-1".to_owned(), "margin-left: 0.25rem;".to_owned());

    let _ = margin.insert("m-1.5".to_owned(), "margin: 0.375rem;".to_owned()); /*0.375rem = 4px */
    let _ = margin.insert("mx-1.5".to_owned(), "margin-left: 0.375rem;margin-right: 0.375rem;".to_owned());
    let _ = margin.insert("my-1.5".to_owned(), "margin-top: 0.375rem;margin-bottom: 0.375rem;".to_owned());
    let _ = margin.insert("mt-1.5".to_owned(), "margin-top: 0.375rem;".to_owned());
    let _ = margin.insert("mr-1.5".to_owned(), "margin-right: 0.375rem;".to_owned());
    let _ = margin.insert("mb-1.5".to_owned(), "margin-bottom: 0.375rem;".to_owned());
    let _ = margin.insert("ml-1.5".to_owned(), "margin-left: 0.375rem;".to_owned());

    let _ = margin.insert("m-2".to_owned(), "margin: 0.5rem;".to_owned()); /*0.5rem = 8px */
    let _ = margin.insert("mx-2".to_owned(), "margin-left: 0.5rem;margin-right: 0.5rem;".to_owned());
    let _ = margin.insert("my-2".to_owned(), "margin-top: 0.5rem;margin-bottom: 0.5rem;".to_owned());
    let _ = margin.insert("mt-2".to_owned(), "margin-top: 0.5rem;".to_owned());
    let _ = margin.insert("mr-2".to_owned(), "margin-right: 0.5rem;".to_owned());
    let _ = margin.insert("mb-2".to_owned(), "margin-bottom: 0.5rem;".to_owned());
    let _ = margin.insert("ml-2".to_owned(), "margin-left: 0.5rem;".to_owned());

    let _ = margin.insert("m-2".to_owned(), "margin: 0.5rem;".to_owned()); /*0.5rem = 8px */
    let _ = margin.insert("mx-2".to_owned(), "margin-left: 0.5rem;margin-right: 0.5rem;".to_owned());
    let _ = margin.insert("my-2".to_owned(), "margin-top: 0.5rem;margin-bottom: 0.5rem;".to_owned());
    let _ = margin.insert("mt-2".to_owned(), "margin-top: 0.5rem;".to_owned());
    let _ = margin.insert("mr-2".to_owned(), "margin-right: 0.5rem;".to_owned());
    let _ = margin.insert("mb-2".to_owned(), "margin-bottom: 0.5rem;".to_owned());
    let _ = margin.insert("ml-2".to_owned(), "margin-left: 0.5rem;".to_owned());

    let _ = margin.insert("m-2.5".to_owned(), "margin: 0.625rem;".to_owned()); /*0.625rem = 10px */
    let _ = margin.insert("mx-2.5".to_owned(), "margin-left: 0.625rem;margin-right: 0.625rem;".to_owned());
    let _ = margin.insert("my-2.5".to_owned(), "margin-top: 0.625rem;margin-bottom: 0.625rem;".to_owned());
    let _ = margin.insert("mt-2.5".to_owned(), "margin-top: 0.625rem;".to_owned());
    let _ = margin.insert("mr-2.5".to_owned(), "margin-right: 0.625rem;".to_owned());
    let _ = margin.insert("mb-2.5".to_owned(), "margin-bottom: 0.625rem;".to_owned());
    let _ = margin.insert("ml-2.5".to_owned(), "margin-left: 0.625rem;".to_owned());

    let _ = margin.insert("m-3".to_owned(), "margin: 0.75rem;".to_owned()); /*0.75rem = 12px */
    let _ = margin.insert("mx-3".to_owned(), "margin-left: 0.75rem;margin-right: 0.75rem;".to_owned());
    let _ = margin.insert("my-3".to_owned(), "margin-top: 0.75rem;margin-bottom: 0.75rem;".to_owned());
    let _ = margin.insert("mt-3".to_owned(), "margin-top: 0.75rem;".to_owned());
    let _ = margin.insert("mr-3".to_owned(), "margin-right: 0.75rem;".to_owned());
    let _ = margin.insert("mb-3".to_owned(), "margin-bottom: 0.75rem;".to_owned());
    let _ = margin.insert("ml-3".to_owned(), "margin-left: 0.75rem;".to_owned());

    let _ = margin.insert("m-3.5".to_owned(), "margin: 0.875rem;".to_owned()); /*0.875rem = 14px */
    let _ = margin.insert("mx-3.5".to_owned(), "margin-left: 0.875rem;margin-right: 0.875rem;".to_owned());
    let _ = margin.insert("my-3.5".to_owned(), "margin-top: 0.875rem;margin-bottom: 0.875rem;".to_owned());
    let _ = margin.insert("mt-3.5".to_owned(), "margin-top: 0.875rem;".to_owned());
    let _ = margin.insert("mr-3.5".to_owned(), "margin-right: 0.875rem;".to_owned());
    let _ = margin.insert("mb-3.5".to_owned(), "margin-bottom: 0.875rem;".to_owned());
    let _ = margin.insert("ml-3.5".to_owned(), "margin-left: 0.875rem;".to_owned());

    let _ = margin.insert("m-4".to_owned(), "margin: 1rem;".to_owned()); /*1rem = 16px */
    let _ = margin.insert("mx-4".to_owned(), "margin-left: 1rem;margin-right: 1rem;".to_owned());
    let _ = margin.insert("my-4".to_owned(), "margin-top: 1rem;margin-bottom: 1rem;".to_owned());
    let _ = margin.insert("mt-4".to_owned(), "margin-top: 1rem;".to_owned());
    let _ = margin.insert("mr-4".to_owned(), "margin-right: 1rem;".to_owned());
    let _ = margin.insert("mb-4".to_owned(), "margin-bottom: 1rem;".to_owned());
    let _ = margin.insert("ml-4".to_owned(), "margin-left: 1rem;".to_owned());

    let _ = margin.insert("m-5".to_owned(), "margin: 1.25rem;".to_owned()); /*1.25rem = 20px */
    let _ = margin.insert("mx-5".to_owned(), "margin-left: 1.25rem;margin-right: 1.25rem;".to_owned());
    let _ = margin.insert("my-5".to_owned(), "margin-top: 1.25rem;margin-bottom: 1.25rem;".to_owned());
    let _ = margin.insert("mt-5".to_owned(), "margin-top: 1.25rem;".to_owned());
    let _ = margin.insert("mr-5".to_owned(), "margin-right: 1.25rem;".to_owned());
    let _ = margin.insert("mb-5".to_owned(), "margin-bottom: 1.25rem;".to_owned());
    let _ = margin.insert("ml-5".to_owned(), "margin-left: 1.25rem;".to_owned());

    let _ = margin.insert("m-6".to_owned(), "margin: 1.5rem;".to_owned()); /*1.5rem = 24px */
    let _ = margin.insert("mx-6".to_owned(), "margin-left: 1.5rem;margin-right: 1.5rem;".to_owned());
    let _ = margin.insert("my-6".to_owned(), "margin-top: 1.5rem;margin-bottom: 1.5rem;".to_owned());
    let _ = margin.insert("mt-6".to_owned(), "margin-top: 1.5rem;".to_owned());
    let _ = margin.insert("mr-6".to_owned(), "margin-right: 1.5rem;".to_owned());
    let _ = margin.insert("mb-6".to_owned(), "margin-bottom: 1.5rem;".to_owned());
    let _ = margin.insert("ml-6".to_owned(), "margin-left: 1.5rem;".to_owned());

    let _ = margin.insert("m-7".to_owned(), "margin: 1.75rem;".to_owned()); /*1.75rem = 28px */
    let _ = margin.insert("mx-7".to_owned(), "margin-left: 1.75rem;margin-right: 1.75rem;".to_owned());
    let _ = margin.insert("my-7".to_owned(), "margin-top: 1.75rem;margin-bottom: 1.75rem;".to_owned());
    let _ = margin.insert("mt-7".to_owned(), "margin-top: 1.75rem;".to_owned());
    let _ = margin.insert("mr-7".to_owned(), "margin-right: 1.75rem;".to_owned());
    let _ = margin.insert("mb-7".to_owned(), "margin-bottom: 1.75rem;".to_owned());
    let _ = margin.insert("ml-7".to_owned(), "margin-left: 1.75rem;".to_owned());

    let _ = margin.insert("m-8".to_owned(), "margin: 2rem;".to_owned()); /*2rem = 32px */
    let _ = margin.insert("mx-8".to_owned(), "margin-left: 2rem;margin-right: 2rem;".to_owned());
    let _ = margin.insert("my-8".to_owned(), "margin-top: 2rem;margin-bottom: 2rem;".to_owned());
    let _ = margin.insert("mt-8".to_owned(), "margin-top: 2rem;".to_owned());
    let _ = margin.insert("mr-8".to_owned(), "margin-right: 2rem;".to_owned());
    let _ = margin.insert("mb-8".to_owned(), "margin-bottom: 2rem;".to_owned());
    let _ = margin.insert("ml-8".to_owned(), "margin-left: 2rem;".to_owned());

    let _ = margin.insert("m-9".to_owned(), "margin: 2.25rem;".to_owned()); /*2.25rem = 36px */
    let _ = margin.insert("mx-9".to_owned(), "margin-left: 2.25rem;margin-right: 2.25rem;".to_owned());
    let _ = margin.insert("my-9".to_owned(), "margin-top: 2.25rem;margin-bottom: 2.25rem;".to_owned());
    let _ = margin.insert("mt-9".to_owned(), "margin-top: 2.25rem;".to_owned());
    let _ = margin.insert("mr-9".to_owned(), "margin-right: 2.25rem;".to_owned());
    let _ = margin.insert("mb-9".to_owned(), "margin-bottom: 2.25rem;".to_owned());
    let _ = margin.insert("ml-9".to_owned(), "margin-left: 2.25rem;".to_owned());

    let _ = margin.insert("m-9".to_owned(), "margin: 2.25rem;".to_owned()); /*2.25rem = 36px */
    let _ = margin.insert("mx-9".to_owned(), "margin-left: 2.25rem;margin-right: 2.25rem;".to_owned());
    let _ = margin.insert("my-9".to_owned(), "margin-top: 2.25rem;margin-bottom: 2.25rem;".to_owned());
    let _ = margin.insert("mt-9".to_owned(), "margin-top: 2.25rem;".to_owned());
    let _ = margin.insert("mr-9".to_owned(), "margin-right: 2.25rem;".to_owned());
    let _ = margin.insert("mb-9".to_owned(), "margin-bottom: 2.25rem;".to_owned());
    let _ = margin.insert("ml-9".to_owned(), "margin-left: 2.25rem;".to_owned());

    let _ = margin.insert("m-10".to_owned(), "margin: 2.5rem;".to_owned()); /*2.5rem = 40px */
    let _ = margin.insert("mx-10".to_owned(), "margin-left: 2.5rem;margin-right: 2.5rem;".to_owned());
    let _ = margin.insert("my-10".to_owned(), "margin-top: 2.5rem;margin-bottom: 2.5rem;".to_owned());
    let _ = margin.insert("mt-10".to_owned(), "margin-top: 2.5rem;".to_owned());
    let _ = margin.insert("mr-10".to_owned(), "margin-right: 2.5rem;".to_owned());
    let _ = margin.insert("mb-10".to_owned(), "margin-bottom: 2.5rem;".to_owned());
    let _ = margin.insert("ml-10".to_owned(), "margin-left: 2.5rem;".to_owned());

    let _ = margin.insert("m-11".to_owned(), "margin: 2.75rem;".to_owned()); /*2.75rem = 44px */
    let _ = margin.insert("mx-11".to_owned(), "margin-left: 2.75rem;margin-right: 2.75rem;".to_owned());
    let _ = margin.insert("my-11".to_owned(), "margin-top: 2.75rem;margin-bottom: 2.75rem;".to_owned());
    let _ = margin.insert("mt-11".to_owned(), "margin-top: 2.75rem;".to_owned());
    let _ = margin.insert("mr-11".to_owned(), "margin-right: 2.75rem;".to_owned());
    let _ = margin.insert("mb-11".to_owned(), "margin-bottom: 2.75rem;".to_owned());
    let _ = margin.insert("ml-11".to_owned(), "margin-left: 2.75rem;".to_owned());

    let _ = margin.insert("m-12".to_owned(), "margin: 3rem;".to_owned()); /*3rem = 48px */
    let _ = margin.insert("mx-12".to_owned(), "margin-left: 3rem;margin-right: 3rem;".to_owned());
    let _ = margin.insert("my-12".to_owned(), "margin-top: 3rem;margin-bottom: 3rem;".to_owned());
    let _ = margin.insert("mt-12".to_owned(), "margin-top: 3rem;".to_owned());
    let _ = margin.insert("mr-12".to_owned(), "margin-right: 3rem;".to_owned());
    let _ = margin.insert("mb-12".to_owned(), "margin-bottom: 3rem;".to_owned());
    let _ = margin.insert("ml-12".to_owned(), "margin-left: 3rem;".to_owned());

    let _ = margin.insert("m-13".to_owned(), "margin: 3.25rem;".to_owned()); /*3.25rem = 52px */
    let _ = margin.insert("mx-13".to_owned(), "margin-left: 3.25rem;margin-right: 3.25rem;".to_owned());
    let _ = margin.insert("my-13".to_owned(), "margin-top: 3.25rem;margin-bottom: 3.25rem;".to_owned());
    let _ = margin.insert("mt-13".to_owned(), "margin-top: 3.25rem;".to_owned());
    let _ = margin.insert("mr-13".to_owned(), "margin-right: 3.25rem;".to_owned());
    let _ = margin.insert("mb-13".to_owned(), "margin-bottom: 3.25rem;".to_owned());
    let _ = margin.insert("ml-13".to_owned(), "margin-left: 3.25rem;".to_owned());

    let _ = margin.insert("m-14".to_owned(), "margin: 3.5rem;".to_owned()); /*3.5rem = 56px */
    let _ = margin.insert("mx-14".to_owned(), "margin-left: 3.5rem;margin-right: 3.5rem;".to_owned());
    let _ = margin.insert("my-14".to_owned(), "margin-top: 3.5rem;margin-bottom: 3.5rem;".to_owned());
    let _ = margin.insert("mt-14".to_owned(), "margin-top: 3.5rem;".to_owned());
    let _ = margin.insert("mr-14".to_owned(), "margin-right: 3.5rem;".to_owned());
    let _ = margin.insert("mb-14".to_owned(), "margin-bottom: 3.5rem;".to_owned());
    let _ = margin.insert("ml-14".to_owned(), "margin-left: 3.5rem;".to_owned());

    let _ = margin.insert("m-15".to_owned(), "margin: 3.75rem;".to_owned()); /*3.75rem = 60px */
    let _ = margin.insert("mx-15".to_owned(), "margin-left: 3.75rem;margin-right: 3.75rem;".to_owned());
    let _ = margin.insert("my-15".to_owned(), "margin-top: 3.75rem;margin-bottom: 3.75rem;".to_owned());
    let _ = margin.insert("mt-15".to_owned(), "margin-top: 3.75rem;".to_owned());
    let _ = margin.insert("mr-15".to_owned(), "margin-right: 3.75rem;".to_owned());
    let _ = margin.insert("mb-15".to_owned(), "margin-bottom: 3.75rem;".to_owned());
    let _ = margin.insert("ml-15".to_owned(), "margin-left: 3.75rem;".to_owned());

    let _ = margin.insert("m-16".to_owned(), "margin: 4rem;".to_owned()); /*4rem = 64px */
    let _ = margin.insert("mx-16".to_owned(), "margin-left: 4rem;margin-right: 4rem;".to_owned());
    let _ = margin.insert("my-16".to_owned(), "margin-top: 4rem;margin-bottom: 4rem;".to_owned());
    let _ = margin.insert("mt-16".to_owned(), "margin-top: 4rem;".to_owned());
    let _ = margin.insert("mr-16".to_owned(), "margin-right: 4rem;".to_owned());
    let _ = margin.insert("mb-16".to_owned(), "margin-bottom: 4rem;".to_owned());
    let _ = margin.insert("ml-16".to_owned(), "margin-left: 4rem;".to_owned());

    let _ = margin.insert("m-20".to_owned(), "margin: 5rem;".to_owned()); /*5rem = 80px */
    let _ = margin.insert("mx-20".to_owned(), "margin-left: 5rem;margin-right: 5rem;".to_owned());
    let _ = margin.insert("my-20".to_owned(), "margin-top: 5rem;margin-bottom: 5rem;".to_owned());
    let _ = margin.insert("mt-20".to_owned(), "margin-top: 5rem;".to_owned());
    let _ = margin.insert("mr-20".to_owned(), "margin-right: 5rem;".to_owned());
    let _ = margin.insert("mb-20".to_owned(), "margin-bottom: 5rem;".to_owned());
    let _ = margin.insert("ml-20".to_owned(), "margin-left: 5rem;".to_owned());

    let _ = margin.insert("m-24".to_owned(), "margin: 6rem;".to_owned()); /*6rem = 80px */
    let _ = margin.insert("mx-24".to_owned(), "margin-left: 6rem;margin-right: 6rem;".to_owned());
    let _ = margin.insert("my-24".to_owned(), "margin-top: 6rem;margin-bottom: 6rem;".to_owned());
    let _ = margin.insert("mt-24".to_owned(), "margin-top: 6rem;".to_owned());
    let _ = margin.insert("mr-24".to_owned(), "margin-right: 6rem;".to_owned());
    let _ = margin.insert("mb-24".to_owned(), "margin-bottom: 6rem;".to_owned());
    let _ = margin.insert("ml-24".to_owned(), "margin-left: 6rem;".to_owned());

    let _ = margin.insert("m-28".to_owned(), "margin: 7rem;".to_owned()); /*7rem = 112px */
    let _ = margin.insert("mx-28".to_owned(), "margin-left: 7rem;margin-right: 7rem;".to_owned());
    let _ = margin.insert("my-28".to_owned(), "margin-top: 7rem;margin-bottom: 7rem;".to_owned());
    let _ = margin.insert("mt-28".to_owned(), "margin-top: 7rem;".to_owned());
    let _ = margin.insert("mr-28".to_owned(), "margin-right: 7rem;".to_owned());
    let _ = margin.insert("mb-28".to_owned(), "margin-bottom: 7rem;".to_owned());
    let _ = margin.insert("ml-28".to_owned(), "margin-left: 7rem;".to_owned());

    let _ = margin.insert("m-32".to_owned(), "margin: 8rem;".to_owned()); /*8rem = 128px */
    let _ = margin.insert("mx-32".to_owned(), "margin-left: 8rem;margin-right: 8rem;".to_owned());
    let _ = margin.insert("my-32".to_owned(), "margin-top: 8rem;margin-bottom: 8rem;".to_owned());
    let _ = margin.insert("mt-32".to_owned(), "margin-top: 8rem;".to_owned());
    let _ = margin.insert("mr-32".to_owned(), "margin-right: 8rem;".to_owned());
    let _ = margin.insert("mb-32".to_owned(), "margin-bottom: 8rem;".to_owned());
    let _ = margin.insert("ml-32".to_owned(), "margin-left: 8rem;".to_owned());

    let _ = margin.insert("m-36".to_owned(), "margin: 9rem;".to_owned()); /*9rem = 144px */
    let _ = margin.insert("mx-36".to_owned(), "margin-left: 9rem;margin-right: 9rem;".to_owned());
    let _ = margin.insert("my-36".to_owned(), "margin-top: 9rem;margin-bottom: 9rem;".to_owned());
    let _ = margin.insert("mt-36".to_owned(), "margin-top: 9rem;".to_owned());
    let _ = margin.insert("mr-36".to_owned(), "margin-right: 9rem;".to_owned());
    let _ = margin.insert("mb-36".to_owned(), "margin-bottom: 9rem;".to_owned());
    let _ = margin.insert("ml-36".to_owned(), "margin-left: 9rem;".to_owned());

    let _ = margin.insert("m-40".to_owned(), "margin: 10rem;".to_owned()); /*10rem = 144px */
    let _ = margin.insert("mx-40".to_owned(), "margin-left: 10rem;margin-right: 10rem;".to_owned());
    let _ = margin.insert("my-40".to_owned(), "margin-top: 10rem;margin-bottom: 10rem;".to_owned());
    let _ = margin.insert("mt-40".to_owned(), "margin-top: 10rem;".to_owned());
    let _ = margin.insert("mr-40".to_owned(), "margin-right: 10rem;".to_owned());
    let _ = margin.insert("mb-40".to_owned(), "margin-bottom: 10rem;".to_owned());
    let _ = margin.insert("ml-40".to_owned(), "margin-left: 10rem;".to_owned());

    let _ = margin.insert("m-44".to_owned(), "margin: 11rem;".to_owned()); /*11rem = 176px */
    let _ = margin.insert("mx-44".to_owned(), "margin-left: 11rem;margin-right: 11rem;".to_owned());
    let _ = margin.insert("my-44".to_owned(), "margin-top: 11rem;margin-bottom: 11rem;".to_owned());
    let _ = margin.insert("mt-44".to_owned(), "margin-top: 11rem;".to_owned());
    let _ = margin.insert("mr-44".to_owned(), "margin-right: 11rem;".to_owned());
    let _ = margin.insert("mb-44".to_owned(), "margin-bottom: 11rem;".to_owned());
    let _ = margin.insert("ml-44".to_owned(), "margin-left: 11rem;".to_owned());

    let _ = margin.insert("m-48".to_owned(), "margin: 12rem;".to_owned()); /*12rem = 176px */
    let _ = margin.insert("mx-48".to_owned(), "margin-left: 12rem;margin-right: 12rem;".to_owned());
    let _ = margin.insert("my-48".to_owned(), "margin-top: 12rem;margin-bottom: 12rem;".to_owned());
    let _ = margin.insert("mt-48".to_owned(), "margin-top: 12rem;".to_owned());
    let _ = margin.insert("mr-48".to_owned(), "margin-right: 12rem;".to_owned());
    let _ = margin.insert("mb-48".to_owned(), "margin-bottom: 12rem;".to_owned());
    let _ = margin.insert("ml-48".to_owned(), "margin-left: 12rem;".to_owned());

    let _ = margin.insert("m-52".to_owned(), "margin: 13rem;".to_owned()); /*13rem = 208px */
    let _ = margin.insert("mx-52".to_owned(), "margin-left: 13rem;margin-right: 13rem;".to_owned());
    let _ = margin.insert("my-52".to_owned(), "margin-top: 13rem;margin-bottom: 13rem;".to_owned());
    let _ = margin.insert("mt-52".to_owned(), "margin-top: 13rem;".to_owned());
    let _ = margin.insert("mr-52".to_owned(), "margin-right: 13rem;".to_owned());
    let _ = margin.insert("mb-52".to_owned(), "margin-bottom: 13rem;".to_owned());
    let _ = margin.insert("ml-52".to_owned(), "margin-left: 13rem;".to_owned());

    let _ = margin.insert("m-56".to_owned(), "margin: 14rem;".to_owned()); /*14rem = 224px */
    let _ = margin.insert("mx-56".to_owned(), "margin-left: 14rem;margin-right: 14rem;".to_owned());
    let _ = margin.insert("my-56".to_owned(), "margin-top: 14rem;margin-bottom: 14rem;".to_owned());
    let _ = margin.insert("mt-56".to_owned(), "margin-top: 14rem;".to_owned());
    let _ = margin.insert("mr-56".to_owned(), "margin-right: 14rem;".to_owned());
    let _ = margin.insert("mb-56".to_owned(), "margin-bottom: 14rem;".to_owned());
    let _ = margin.insert("ml-56".to_owned(), "margin-left: 14rem;".to_owned());

    let _ = margin.insert("m-60".to_owned(), "margin: 15rem;".to_owned()); /*15rem = 224px */
    let _ = margin.insert("mx-60".to_owned(), "margin-left: 15rem;margin-right: 15rem;".to_owned());
    let _ = margin.insert("my-60".to_owned(), "margin-top: 15rem;margin-bottom: 15rem;".to_owned());
    let _ = margin.insert("mt-60".to_owned(), "margin-top: 15rem;".to_owned());
    let _ = margin.insert("mr-60".to_owned(), "margin-right: 15rem;".to_owned());
    let _ = margin.insert("mb-60".to_owned(), "margin-bottom: 15rem;".to_owned());
    let _ = margin.insert("ml-60".to_owned(), "margin-left: 15rem;".to_owned());

    let _ = margin.insert("m-64".to_owned(), "margin: 16rem;".to_owned()); /*16rem = 256px */
    let _ = margin.insert("mx-64".to_owned(), "margin-left: 16rem;margin-right: 16rem;".to_owned());
    let _ = margin.insert("my-64".to_owned(), "margin-top: 16rem;margin-bottom: 16rem;".to_owned());
    let _ = margin.insert("mt-64".to_owned(), "margin-top: 16rem;".to_owned());
    let _ = margin.insert("mr-64".to_owned(), "margin-right: 16rem;".to_owned());
    let _ = margin.insert("mb-64".to_owned(), "margin-bottom: 16rem;".to_owned());
    let _ = margin.insert("ml-64".to_owned(), "margin-left: 16rem;".to_owned());

    let _ = margin.insert("m-72".to_owned(), "margin: 18rem;".to_owned()); /*18rem = 288px */
    let _ = margin.insert("mx-72".to_owned(), "margin-left: 18rem;margin-right: 18rem;".to_owned());
    let _ = margin.insert("my-72".to_owned(), "margin-top: 18rem;margin-bottom: 18rem;".to_owned());
    let _ = margin.insert("mt-72".to_owned(), "margin-top: 18rem;".to_owned());
    let _ = margin.insert("mr-72".to_owned(), "margin-right: 18rem;".to_owned());
    let _ = margin.insert("mb-72".to_owned(), "margin-bottom: 18rem;".to_owned());
    let _ = margin.insert("ml-72".to_owned(), "margin-left: 18rem;".to_owned());

    let _ = margin.insert("m-80".to_owned(), "margin: 20rem;".to_owned()); /*20rem = 320px */
    let _ = margin.insert("mx-80".to_owned(), "margin-left: 20rem;margin-right: 20rem;".to_owned());
    let _ = margin.insert("my-80".to_owned(), "margin-top: 20rem;margin-bottom: 20rem;".to_owned());
    let _ = margin.insert("mt-80".to_owned(), "margin-top: 20rem;".to_owned());
    let _ = margin.insert("mr-80".to_owned(), "margin-right: 20rem;".to_owned());
    let _ = margin.insert("mb-80".to_owned(), "margin-bottom: 20rem;".to_owned());
    let _ = margin.insert("ml-80".to_owned(), "margin-left: 20rem;".to_owned());

    let _ = margin.insert("m-96".to_owned(), "margin: 24rem;".to_owned()); /*24rem = 384px */
    let _ = margin.insert("mx-96".to_owned(), "margin-left: 24rem;margin-right: 24rem;".to_owned());
    let _ = margin.insert("my-96".to_owned(), "margin-top: 24rem;margin-bottom: 24rem;".to_owned());
    let _ = margin.insert("mt-96".to_owned(), "margin-top: 24rem;".to_owned());
    let _ = margin.insert("mr-96".to_owned(), "margin-right: 24rem;".to_owned());
    let _ = margin.insert("mb-96".to_owned(), "margin-bottom: 24rem;".to_owned());
    let _ = margin.insert("ml-96".to_owned(), "margin-left: 24rem;".to_owned());

    let _ = margin.insert("m-auto".to_owned(), "margin: auto;".to_owned());
    let _ = margin.insert("mx-auto".to_owned(), "margin-left: auto;margin-right: auto;".to_owned());
    let _ = margin.insert("my-auto".to_owned(), "margin-top: auto;margin-bottom: auto;".to_owned());
    let _ = margin.insert("mt-auto".to_owned(), "margin-top: auto;".to_owned());
    let _ = margin.insert("mr-auto".to_owned(), "margin-right: auto;".to_owned());
    let _ = margin.insert("mb-auto".to_owned(), "margin-bottom: auto;".to_owned());
    let _ = margin.insert("ml-auto".to_owned(), "margin-left: auto;".to_owned());
    
    margin
}
