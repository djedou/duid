use std::collections::HashMap;


pub(crate) fn height() -> HashMap<String, String> {
    let mut height = HashMap::new();
    let _ = height.insert("h-0".to_owned(), "height: 0px;".to_owned());
    let _ = height.insert("h-px".to_owned(), "height: 1px;".to_owned());
    let _ = height.insert("h-0.5".to_owned(), "height: 0.125rem;".to_owned()); /* 2px */
    let _ = height.insert("h-1".to_owned(), "height: 0.25rem;".to_owned()); /* 4px */
    let _ = height.insert("h-1.5".to_owned(), "height: 0.375rem;".to_owned()); /* 6px */
    let _ = height.insert("h-2".to_owned(), "height: 0.5rem;".to_owned()); /* 8px */
    let _ = height.insert("h-2.5".to_owned(), "height: 0.625rem;".to_owned()); /* 10px */
    let _ = height.insert("h-3".to_owned(), "height: 0.75rem;".to_owned()); /* 12px */
    let _ = height.insert("h-3.5".to_owned(), "height: 0.875rem;".to_owned()); /* 14px */
    let _ = height.insert("h-4".to_owned(), "height: 1rem;".to_owned()); /* 16px */
    let _ = height.insert("h-5".to_owned(), "height: 1.25rem;".to_owned()); /* 20px */
    let _ = height.insert("h-6".to_owned(), "height: 1.5rem;".to_owned()); /* 24px */
    let _ = height.insert("h-7".to_owned(), "height: 1.75rem;".to_owned()); /* 28px */
    let _ = height.insert("h-8".to_owned(), "height: 2rem;".to_owned()); /* 32px */
    let _ = height.insert("h-9".to_owned(), "height: 2.25rem;".to_owned()); /* 36px */
    let _ = height.insert("h-10".to_owned(), "height: 2.5rem;".to_owned()); /* 40px */
    let _ = height.insert("h-11".to_owned(), "height: 2.75rem;".to_owned()); /* 44px */
    let _ = height.insert("h-12".to_owned(), "height: 3rem;".to_owned()); /* 48px */
    let _ = height.insert("h-14".to_owned(), "height: 3.5rem;".to_owned()); /* 56px */
    let _ = height.insert("h-16".to_owned(), "height: 4rem;".to_owned()); /* 64px */
    let _ = height.insert("h-20".to_owned(), "height: 5rem;".to_owned()); /* 80px */
    let _ = height.insert("h-24".to_owned(), "height: 6rem;".to_owned()); /* 96px */
    let _ = height.insert("h-28".to_owned(), "height: 7rem;".to_owned()); /* 112px */
    let _ = height.insert("h-32".to_owned(), "height: 8rem;".to_owned()); /* 128px */
    let _ = height.insert("h-36".to_owned(), "height: 9rem;".to_owned()); /* 144px */
    let _ = height.insert("h-40".to_owned(), "height: 10rem;".to_owned()); /* 160px */
    let _ = height.insert("h-44".to_owned(), "height: 11rem;".to_owned()); /* 176px */
    let _ = height.insert("h-48".to_owned(), "height: 12rem;".to_owned()); /* 192px */
    let _ = height.insert("h-52".to_owned(), "height: 13rem;".to_owned()); /* 208px */
    let _ = height.insert("h-56".to_owned(), "height: 14rem;".to_owned()); /* 224px */
    let _ = height.insert("h-60".to_owned(), "height: 15rem;".to_owned()); /* 240px */
    let _ = height.insert("h-64".to_owned(), "height: 16rem;".to_owned()); /* 256px */
    let _ = height.insert("h-72".to_owned(), "height: 18rem;".to_owned()); /* 288px */
    let _ = height.insert("h-80".to_owned(), "height: 20rem;".to_owned()); /* 320px */
    let _ = height.insert("h-96".to_owned(), "height: 24rem;".to_owned()); /* 384px */
    let _ = height.insert("h-1/2".to_owned(), "height: 50%;".to_owned());
    let _ = height.insert("h-1/3".to_owned(), "height: 33.333333%;".to_owned());
    let _ = height.insert("h-2/3".to_owned(), "height: 66.666667%;".to_owned());
    let _ = height.insert("h-1/4".to_owned(), "height: 25%;".to_owned());
    let _ = height.insert("h-2/4".to_owned(), "height: 50%;".to_owned());
    let _ = height.insert("h-3/4".to_owned(), "height: 75%;".to_owned());
    let _ = height.insert("h-1/5".to_owned(), "height: 20%;".to_owned());
    let _ = height.insert("h-2/5".to_owned(), "height: 40%;".to_owned());
    let _ = height.insert("h-3/5".to_owned(), "height: 60%;".to_owned());
    let _ = height.insert("h-4/5".to_owned(), "height: 80%;".to_owned());
    let _ = height.insert("h-1/6".to_owned(), "height: 16.666667%;".to_owned());
    let _ = height.insert("h-2/6".to_owned(), "height: 33.333333%;".to_owned());
    let _ = height.insert("h-3/6".to_owned(), "height: 50%;".to_owned());
    let _ = height.insert("h-4/6".to_owned(), "height: 66.666667%;".to_owned());
    let _ = height.insert("h-5/6".to_owned(), "height: 83.333333%;".to_owned());
    let _ = height.insert("h-full".to_owned(), "height: 100%;".to_owned());
    let _ = height.insert("vh-10".to_owned(), "height: 10vh;".to_owned());
    let _ = height.insert("vh-20".to_owned(), "height: 20vh;".to_owned());
    let _ = height.insert("vh-25".to_owned(), "height: 25vh;".to_owned());
    let _ = height.insert("vh-30".to_owned(), "height: 30vh;".to_owned());
    let _ = height.insert("vh-40".to_owned(), "height: 40vh;".to_owned());
    let _ = height.insert("vh-50".to_owned(), "height: 50vh;".to_owned());
    let _ = height.insert("vh-60".to_owned(), "height: 60vh;".to_owned());
    let _ = height.insert("vh-70".to_owned(), "height: 70vh;".to_owned());
    let _ = height.insert("vh-75".to_owned(), "height: 75vh;".to_owned());
    let _ = height.insert("vh-80".to_owned(), "height: 80vh;".to_owned());
    let _ = height.insert("vh-90".to_owned(), "height: 90vh;".to_owned());
    let _ = height.insert("vh-100".to_owned(), "height: 100vh;".to_owned());
    let _ = height.insert("h-min".to_owned(), "height: min-content;".to_owned());
    let _ = height.insert("h-max".to_owned(), "height: max-content;".to_owned());
    let _ = height.insert("h-fit".to_owned(), "height: fit-content;".to_owned());
    let _ = height.insert("h-auto".to_owned(), "height: auto;".to_owned());
    let _ = height.insert("h-inherit".to_owned(), "height: inherit;".to_owned());

    height
}
