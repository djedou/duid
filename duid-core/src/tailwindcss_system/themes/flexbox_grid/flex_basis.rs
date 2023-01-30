use std::collections::HashMap;


pub(crate) fn flex_basis() -> HashMap<String, String> {
    let mut flex_basis = HashMap::new();
    let _ = flex_basis.insert("basis-0".to_owned(), "flex-basis: 0px;".to_owned());
    let _ = flex_basis.insert("basis-1".to_owned(), "flex-basis: 0.25rem;".to_owned()); /* 4px */
    let _ = flex_basis.insert("basis-2".to_owned(), "flex-basis: 0.5rem;".to_owned()); /* 8px */
    let _ = flex_basis.insert("basis-3".to_owned(), "flex-basis: 0.75rem;".to_owned()); /* 12px */
    let _ = flex_basis.insert("basis-4".to_owned(), "flex-basis: 1rem;".to_owned()); /* 16px */
    let _ = flex_basis.insert("basis-5".to_owned(), "flex-basis: 1.25rem;".to_owned()); /* 20px */
    let _ = flex_basis.insert("basis-6".to_owned(), "flex-basis: 1.5rem;".to_owned()); /* 24px */
    let _ = flex_basis.insert("basis-7".to_owned(), "flex-basis: 1.75rem;".to_owned()); /* 28px */
    let _ = flex_basis.insert("basis-8".to_owned(), "flex-basis: 2rem;".to_owned()); /* 32px */
    let _ = flex_basis.insert("basis-9".to_owned(), "flex-basis: 2.25rem;".to_owned()); /* 36px */
    let _ = flex_basis.insert("basis-10".to_owned(), "flex-basis: 2.5rem;".to_owned()); /* 40px */
    let _ = flex_basis.insert("basis-11".to_owned(), "flex-basis: 2.75rem;".to_owned()); /* 44px */
    let _ = flex_basis.insert("basis-12".to_owned(), "flex-basis: 3rem;".to_owned()); /* 48px */
    let _ = flex_basis.insert("basis-14".to_owned(), "flex-basis: 3.5rem;".to_owned()); /* 56px */
    let _ = flex_basis.insert("basis-16".to_owned(), "flex-basis: 4rem;".to_owned()); /* 64px */
    let _ = flex_basis.insert("basis-20".to_owned(), "flex-basis: 5rem;".to_owned()); /* 80px */
    let _ = flex_basis.insert("basis-24".to_owned(), "flex-basis: 6rem;".to_owned()); /* 96px */
    let _ = flex_basis.insert("basis-28".to_owned(), "flex-basis: 7rem;".to_owned()); /* 112px */
    let _ = flex_basis.insert("basis-32".to_owned(), "flex-basis: 8rem;".to_owned()); /* 128px */
    let _ = flex_basis.insert("basis-36".to_owned(), "flex-basis: 9rem;".to_owned()); /* 144px */
    let _ = flex_basis.insert("basis-40".to_owned(), "flex-basis: 10rem;".to_owned()); /* 160px */
    let _ = flex_basis.insert("basis-44".to_owned(), "flex-basis: 11rem;".to_owned()); /* 176px */
    let _ = flex_basis.insert("basis-48".to_owned(), "flex-basis: 12rem;".to_owned()); /* 192px */
    let _ = flex_basis.insert("basis-52".to_owned(), "flex-basis: 13rem;".to_owned()); /* 208px */
    let _ = flex_basis.insert("basis-56".to_owned(), "flex-basis: 14rem;".to_owned()); /* 224px */
    let _ = flex_basis.insert("basis-60".to_owned(), "flex-basis: 15rem;".to_owned()); /* 240px */
    let _ = flex_basis.insert("basis-64".to_owned(), "flex-basis: 16rem;".to_owned()); /* 256px */
    let _ = flex_basis.insert("basis-72".to_owned(), "flex-basis: 18rem;".to_owned()); /* 288px */
    let _ = flex_basis.insert("basis-80".to_owned(), "flex-basis: 20rem;".to_owned()); /* 320px */
    let _ = flex_basis.insert("basis-96".to_owned(), "flex-basis: 24rem;".to_owned()); /* 384px */
    let _ = flex_basis.insert("basis-auto".to_owned(), "flex-basis: auto;".to_owned());
    let _ = flex_basis.insert("basis-px".to_owned(), "flex-basis: 1px;".to_owned());
    let _ = flex_basis.insert("basis-0.5".to_owned(), "flex-basis: 0.125rem;".to_owned()); /* 2px */
    let _ = flex_basis.insert("basis-1.5".to_owned(), "flex-basis: 0.375rem;".to_owned()); /* 6px */
    let _ = flex_basis.insert("basis-2.5".to_owned(), "flex-basis: 0.625rem;".to_owned()); /* 10px */
    let _ = flex_basis.insert("basis-3.5".to_owned(), "flex-basis: 0.875rem;".to_owned()); /* 14px */
    let _ = flex_basis.insert("basis-1/2".to_owned(), "flex-basis: 50%;".to_owned());
    let _ = flex_basis.insert("basis-1/3".to_owned(), "flex-basis: 33.333333%;".to_owned());
    let _ = flex_basis.insert("basis-2/3".to_owned(), "flex-basis: 66.666667%;".to_owned());
    let _ = flex_basis.insert("basis-1/4".to_owned(), "flex-basis: 25%;".to_owned());
    let _ = flex_basis.insert("basis-2/4".to_owned(), "flex-basis: 50%;".to_owned());
    let _ = flex_basis.insert("basis-3/4".to_owned(), "flex-basis: 75%;".to_owned());
    let _ = flex_basis.insert("basis-1/5".to_owned(), "flex-basis: 20%;".to_owned());
    let _ = flex_basis.insert("basis-2/5".to_owned(), "flex-basis: 40%;".to_owned());
    let _ = flex_basis.insert("basis-3/5".to_owned(), "flex-basis: 60%;".to_owned());
    let _ = flex_basis.insert("basis-4/5".to_owned(), "flex-basis: 80%;".to_owned());
    let _ = flex_basis.insert("basis-1/6".to_owned(), "flex-basis: 16.666667%;".to_owned());
    let _ = flex_basis.insert("basis-2/6".to_owned(), "flex-basis: 33.333333%;".to_owned());
    let _ = flex_basis.insert("basis-3/6".to_owned(), "flex-basis: 50%;".to_owned());
    let _ = flex_basis.insert("basis-4/6".to_owned(), "flex-basis: 66.666667%;".to_owned());
    let _ = flex_basis.insert("basis-5/6".to_owned(), "flex-basis: 83.333333%;".to_owned());
    let _ = flex_basis.insert("basis-1/12".to_owned(), "flex-basis: 8.333333%;".to_owned());
    let _ = flex_basis.insert("basis-2/12".to_owned(), "flex-basis: 16.666667%;".to_owned());
    let _ = flex_basis.insert("basis-3/12".to_owned(), "flex-basis: 25%;".to_owned());
    let _ = flex_basis.insert("basis-4/12".to_owned(), "flex-basis: 33.333333%;".to_owned());
    let _ = flex_basis.insert("basis-5/12".to_owned(), "flex-basis: 41.666667%;".to_owned());
    let _ = flex_basis.insert("basis-6/12".to_owned(), "flex-basis: 50%;".to_owned());
    let _ = flex_basis.insert("basis-7/12".to_owned(), "flex-basis: 58.333333%;".to_owned());
    let _ = flex_basis.insert("basis-8/12".to_owned(), "flex-basis: 66.666667%;".to_owned());
    let _ = flex_basis.insert("basis-9/12".to_owned(), "flex-basis: 75%;".to_owned());
    let _ = flex_basis.insert("basis-10/12".to_owned(), "flex-basis: 83.333333%;".to_owned());
    let _ = flex_basis.insert("basis-11/12".to_owned(), "flex-basis: 91.666667%;".to_owned());
    let _ = flex_basis.insert("basis-full".to_owned(), "flex-basis: 100%;".to_owned());
   


    flex_basis
}