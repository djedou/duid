use std::collections::HashMap;


pub(crate) fn top_right_bottom_left() -> HashMap<String, String> {
    let mut top_right_bottom_left = HashMap::new();

    let _ = top_right_bottom_left.insert("inset-0".to_owned(), "top: 0px;right: 0px;bottom: 0px;left: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-0".to_owned(), "left: 0px;right: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-0".to_owned(), "top: 0px;bottom: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("top-0".to_owned(), "top: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("right-0".to_owned(), "right: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-0".to_owned(), "bottom: 0px;".to_owned());
    let _ = top_right_bottom_left.insert("left-0".to_owned(), "left: 0px;".to_owned());

    let _ = top_right_bottom_left.insert("inset-px".to_owned(), "top: 1px;right: 1px;bottom: 1px;left: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-px".to_owned(), "left: 1px;right: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-px".to_owned(), "top: 1px;bottom: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("top-px".to_owned(), "top: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("right-px".to_owned(), "right: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-px".to_owned(), "bottom: 1px;".to_owned());
    let _ = top_right_bottom_left.insert("left-px".to_owned(), "left: 1px;".to_owned());

    let _ = top_right_bottom_left.insert("inset-0.5".to_owned(), "top: 0.125rem;right: 0.125rem;bottom: 0.125rem;left: 0.125rem;".to_owned());/* 0.125rem = 2px*/
    let _ = top_right_bottom_left.insert("inset-x-0.5".to_owned(), "left: 0.125rem;right: 0.125rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-0.5".to_owned(), "top: 0.125rem;bottom: 0.125rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-0.5".to_owned(), "top: 0.125rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-0.5".to_owned(), "right: 0.125rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-0.5".to_owned(), "bottom: 0.125rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-0.5".to_owned(), "left: 0.125rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-1".to_owned(), "top: 0.25rem;right: 0.25rem;bottom: 0.25rem;left: 0.25rem;".to_owned());/* 0.25rem = 4px*/
    let _ = top_right_bottom_left.insert("inset-x-1".to_owned(), "left: 0.25rem;right: 0.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-1".to_owned(), "top: 0.25rem;bottom: 0.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-1".to_owned(), "top: 0.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-1".to_owned(), "right: 0.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-1".to_owned(), "bottom: 0.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-1".to_owned(), "left: 0.25rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-1.5".to_owned(), "top: 0.375rem;right: 0.375rem;bottom: 0.375rem;left: 0.375rem;".to_owned());/* 0.375rem = 6px*/
    let _ = top_right_bottom_left.insert("inset-x-1.5".to_owned(), "left: 0.375rem;right: 0.375rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-1.5".to_owned(), "top: 0.375rem;bottom: 0.375rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-1.5".to_owned(), "top: 0.375rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-1.5".to_owned(), "right: 0.375rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-1.5".to_owned(), "bottom: 0.375rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-1.5".to_owned(), "left: 0.375rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-2".to_owned(), "top: 0.5rem;right: 0.5rem;bottom: 0.5rem;left: 0.5rem;".to_owned());/* 0.5rem = 8px*/
    let _ = top_right_bottom_left.insert("inset-x-2".to_owned(), "left: 0.5rem;right: 0.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-2".to_owned(), "top: 0.5rem;bottom: 0.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-2".to_owned(), "top: 0.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-2".to_owned(), "right: 0.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-2".to_owned(), "bottom: 0.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-2".to_owned(), "left: 0.5rem;".to_owned());
    
    let _ = top_right_bottom_left.insert("inset-2.5".to_owned(), "top: 0.625rem;right: 0.625rem;bottom: 0.625rem;left: 0.625rem;".to_owned());/* 0.625rem = 10px*/
    let _ = top_right_bottom_left.insert("inset-x-2.5".to_owned(), "left: 0.625rem;right: 0.625rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-2.5".to_owned(), "top: 0.625rem;bottom: 0.625rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-2.5".to_owned(), "top: 0.625rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-2.5".to_owned(), "right: 0.625rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-2.5".to_owned(), "bottom: 0.625rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-2.5".to_owned(), "left: 0.625rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-3".to_owned(), "top: 0.75rem;right: 0.75rem;bottom: 0.75rem;left: 0.75rem;".to_owned());/* 0.75rem = 12px*/
    let _ = top_right_bottom_left.insert("inset-x-3".to_owned(), "left: 0.75rem;right: 0.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-3".to_owned(), "top: 0.75rem;bottom: 0.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-3".to_owned(), "top: 0.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-3".to_owned(), "right: 0.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-3".to_owned(), "bottom: 0.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-3".to_owned(), "left: 0.75rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-3.5".to_owned(), "top: 0.875rem;right: 0.875rem;bottom: 0.875rem;left: 0.875rem;".to_owned());/* 0.875rem = 14px*/
    let _ = top_right_bottom_left.insert("inset-x-3.5".to_owned(), "left: 0.875rem;right: 0.875rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-3.5".to_owned(), "top: 0.875rem;bottom: 0.875rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-3.5".to_owned(), "top: 0.875rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-3.5".to_owned(), "right: 0.875rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-3.5".to_owned(), "bottom: 0.875rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-3.5".to_owned(), "left: 0.875rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-4".to_owned(), "top: 1rem;right: 1rem;bottom: 1rem;left: 1rem;".to_owned());/* 1rem = 16px*/
    let _ = top_right_bottom_left.insert("inset-x-4".to_owned(), "left: 1rem;right: 1rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-4".to_owned(), "top: 1rem;bottom: 1rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-4".to_owned(), "top: 1rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-4".to_owned(), "right: 1rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-4".to_owned(), "bottom: 1rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-4".to_owned(), "left: 1rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-5".to_owned(), "top: 1.25rem;right: 1.25rem;bottom: 1.25rem;left: 1.25rem;".to_owned());/* 1.25rem = 20px*/
    let _ = top_right_bottom_left.insert("inset-x-5".to_owned(), "left: 1.25rem;right: 1.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-5".to_owned(), "top: 1.25rem;bottom: 1.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-5".to_owned(), "top: 1.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-5".to_owned(), "right: 1.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-5".to_owned(), "bottom: 1.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-5".to_owned(), "left: 1.25rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-6".to_owned(), "top: 1.5rem;right: 1.5rem;bottom: 1.5rem;left: 1.5rem;".to_owned());/* 1.5rem = 24px*/
    let _ = top_right_bottom_left.insert("inset-x-6".to_owned(), "left: 1.5rem;right: 1.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-6".to_owned(), "top: 1.5rem;bottom: 1.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-6".to_owned(), "top: 1.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-6".to_owned(), "right: 1.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-6".to_owned(), "bottom: 1.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-6".to_owned(), "left: 1.5rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-7".to_owned(), "top: 1.75rem;right: 1.75rem;bottom: 1.75rem;left: 1.75rem;".to_owned());/* 1.75rem = 28px*/
    let _ = top_right_bottom_left.insert("inset-x-7".to_owned(), "left: 1.75rem;right: 1.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-7".to_owned(), "top: 1.75rem;bottom: 1.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-7".to_owned(), "top: 1.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-7".to_owned(), "right: 1.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-7".to_owned(), "bottom: 1.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-7".to_owned(), "left: 1.75rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-8".to_owned(), "top: 2rem;right: 2rem;bottom: 2rem;left: 2rem;".to_owned());/* 2rem = 32px*/
    let _ = top_right_bottom_left.insert("inset-x-8".to_owned(), "left: 2rem;right: 2rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-8".to_owned(), "top: 2rem;bottom: 2rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-8".to_owned(), "top: 2rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-8".to_owned(), "right: 2rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-8".to_owned(), "bottom: 2rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-8".to_owned(), "left: 2rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-9".to_owned(), "top: 2.25rem;right: 2.25rem;bottom: 2.25rem;left: 2.25rem;".to_owned());/* 2.25rem = 36px*/
    let _ = top_right_bottom_left.insert("inset-x-9".to_owned(), "left: 2.25rem;right: 2.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-9".to_owned(), "top: 2.25rem;bottom: 2.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-9".to_owned(), "top: 2.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-9".to_owned(), "right: 2.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-9".to_owned(), "bottom: 2.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-9".to_owned(), "left: 2.25rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-10".to_owned(), "top: 2.5rem;right: 2.5rem;bottom: 2.5rem;left: 2.5rem;".to_owned());/* 2.5rem = 40px*/
    let _ = top_right_bottom_left.insert("inset-x-10".to_owned(), "left: 2.5rem;right: 2.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-10".to_owned(), "top: 2.5rem;bottom: 2.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-10".to_owned(), "top: 2.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-10".to_owned(), "right: 2.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-10".to_owned(), "bottom: 2.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-10".to_owned(), "left: 2.5rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-11".to_owned(), "top: 2.75rem;right: 2.75rem;bottom: 2.75rem;left: 2.75rem;".to_owned());/* 2.75rem = 44px*/
    let _ = top_right_bottom_left.insert("inset-x-11".to_owned(), "left: 2.75rem;right: 2.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-11".to_owned(), "top: 2.75rem;bottom: 2.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-11".to_owned(), "top: 2.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-11".to_owned(), "right: 2.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-11".to_owned(), "bottom: 2.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-11".to_owned(), "left: 2.75rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-12".to_owned(), "top: 3rem;right: 3rem;bottom: 3rem;left: 3rem;".to_owned());/* 3rem = 48px*/
    let _ = top_right_bottom_left.insert("inset-x-12".to_owned(), "left: 3rem;right: 3rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-12".to_owned(), "top: 3rem;bottom: 3rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-12".to_owned(), "top: 3rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-12".to_owned(), "right: 3rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-12".to_owned(), "bottom: 3rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-12".to_owned(), "left: 3rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-13".to_owned(), "top: 3.25rem;right: 3.25rem;bottom: 3.25rem;left: 3.25rem;".to_owned());/* 3.25rem = 52px*/
    let _ = top_right_bottom_left.insert("inset-x-13".to_owned(), "left: 3.25rem;right: 3.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-13".to_owned(), "top: 3.25rem;bottom: 3.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-13".to_owned(), "top: 3.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-13".to_owned(), "right: 3.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-13".to_owned(), "bottom: 3.25rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-13".to_owned(), "left: 3.25rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-14".to_owned(), "top: 3.5rem;right: 3.5rem;bottom: 3.5rem;left: 3.5rem;".to_owned());/* 3.5rem = 56px*/
    let _ = top_right_bottom_left.insert("inset-x-14".to_owned(), "left: 3.5rem;right: 3.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-14".to_owned(), "top: 3.5rem;bottom: 3.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-14".to_owned(), "top: 3.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-14".to_owned(), "right: 3.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-14".to_owned(), "bottom: 3.5rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-14".to_owned(), "left: 3.5rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-15".to_owned(), "top: 3.75rem;right: 3.75rem;bottom: 3.75rem;left: 3.75rem;".to_owned());/* 3.75rem = 60px*/
    let _ = top_right_bottom_left.insert("inset-x-15".to_owned(), "left: 3.75rem;right: 3.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-15".to_owned(), "top: 3.75rem;bottom: 3.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-15".to_owned(), "top: 3.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-15".to_owned(), "right: 3.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-15".to_owned(), "bottom: 3.75rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-15".to_owned(), "left: 3.75rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-16".to_owned(), "top: 4rem;right: 4rem;bottom: 4rem;left: 4rem;".to_owned());/* 4rem = 64px*/
    let _ = top_right_bottom_left.insert("inset-x-16".to_owned(), "left: 4rem;right: 4rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-16".to_owned(), "top: 4rem;bottom: 4rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-16".to_owned(), "top: 4rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-16".to_owned(), "right: 4rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-16".to_owned(), "bottom: 4rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-16".to_owned(), "left: 4rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-20".to_owned(), "top: 5rem;right: 5rem;bottom: 5rem;left: 5rem;".to_owned());/* 5rem = 80px*/
    let _ = top_right_bottom_left.insert("inset-x-20".to_owned(), "left: 5rem;right: 5rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-20".to_owned(), "top: 5rem;bottom: 5rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-20".to_owned(), "top: 5rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-20".to_owned(), "right: 5rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-20".to_owned(), "bottom: 5rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-20".to_owned(), "left: 5rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-24".to_owned(), "top: 6rem;right: 6rem;bottom: 6rem;left: 6rem;".to_owned());/* 6rem = 96px*/
    let _ = top_right_bottom_left.insert("inset-x-24".to_owned(), "left: 6rem;right: 6rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-24".to_owned(), "top: 6rem;bottom: 6rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-24".to_owned(), "top: 6rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-24".to_owned(), "right: 6rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-24".to_owned(), "bottom: 6rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-24".to_owned(), "left: 6rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-28".to_owned(), "top: 7rem;right: 7rem;bottom: 7rem;left: 7rem;".to_owned());/* 7rem = 112px*/
    let _ = top_right_bottom_left.insert("inset-x-28".to_owned(), "left: 7rem;right: 7rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-28".to_owned(), "top: 7rem;bottom: 7rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-28".to_owned(), "top: 7rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-28".to_owned(), "right: 7rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-28".to_owned(), "bottom: 7rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-28".to_owned(), "left: 7rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-32".to_owned(), "top: 8rem;right: 8rem;bottom: 8rem;left: 8rem;".to_owned());/* 8rem = 128px*/
    let _ = top_right_bottom_left.insert("inset-x-32".to_owned(), "left: 8rem;right: 8rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-32".to_owned(), "top: 8rem;bottom: 8rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-32".to_owned(), "top: 8rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-32".to_owned(), "right: 8rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-32".to_owned(), "bottom: 8rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-32".to_owned(), "left: 8rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-36".to_owned(), "top: 9rem;right: 9rem;bottom: 9rem;left: 9rem;".to_owned());/* 9rem = 144px*/
    let _ = top_right_bottom_left.insert("inset-x-36".to_owned(), "left: 9rem;right: 9rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-36".to_owned(), "top: 9rem;bottom: 9rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-36".to_owned(), "top: 9rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-36".to_owned(), "right: 9rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-36".to_owned(), "bottom: 9rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-36".to_owned(), "left: 9rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-40".to_owned(), "top: 10rem;right: 10rem;bottom: 10rem;left: 10rem;".to_owned());/* 10rem = 160px*/
    let _ = top_right_bottom_left.insert("inset-x-40".to_owned(), "left: 10rem;right: 10rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-40".to_owned(), "top: 10rem;bottom: 10rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-40".to_owned(), "top: 10rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-40".to_owned(), "right: 10rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-40".to_owned(), "bottom: 10rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-40".to_owned(), "left: 10rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-44".to_owned(), "top: 11rem;right: 11rem;bottom: 11rem;left: 11rem;".to_owned());/* 11rem = 176px*/
    let _ = top_right_bottom_left.insert("inset-x-44".to_owned(), "left: 11rem;right: 11rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-44".to_owned(), "top: 11rem;bottom: 11rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-44".to_owned(), "top: 11rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-44".to_owned(), "right: 11rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-44".to_owned(), "bottom: 11rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-44".to_owned(), "left: 11rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-48".to_owned(), "top: 12rem;right: 12rem;bottom: 12rem;left: 12rem;".to_owned());/* 12rem = 192px*/
    let _ = top_right_bottom_left.insert("inset-x-48".to_owned(), "left: 12rem;right: 12rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-48".to_owned(), "top: 12rem;bottom: 12rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-48".to_owned(), "top: 12rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-48".to_owned(), "right: 12rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-48".to_owned(), "bottom: 12rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-48".to_owned(), "left: 12rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-52".to_owned(), "top: 13rem;right: 13rem;bottom: 13rem;left: 13rem;".to_owned());/* 13rem = 208px*/
    let _ = top_right_bottom_left.insert("inset-x-52".to_owned(), "left: 13rem;right: 13rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-52".to_owned(), "top: 13rem;bottom: 13rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-52".to_owned(), "top: 13rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-52".to_owned(), "right: 13rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-52".to_owned(), "bottom: 13rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-52".to_owned(), "left: 13rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-56".to_owned(), "top: 14rem;right: 14rem;bottom: 14rem;left: 14rem;".to_owned());/* 14rem = 224px*/
    let _ = top_right_bottom_left.insert("inset-x-56".to_owned(), "left: 14rem;right: 14rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-56".to_owned(), "top: 14rem;bottom: 14rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-56".to_owned(), "top: 14rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-56".to_owned(), "right: 14rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-56".to_owned(), "bottom: 14rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-56".to_owned(), "left: 14rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-60".to_owned(), "top: 15rem;right: 15rem;bottom: 15rem;left: 15rem;".to_owned());/* 15rem = 240px*/
    let _ = top_right_bottom_left.insert("inset-x-60".to_owned(), "left: 15rem;right: 15rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-60".to_owned(), "top: 15rem;bottom: 15rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-60".to_owned(), "top: 15rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-60".to_owned(), "right: 15rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-60".to_owned(), "bottom: 15rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-60".to_owned(), "left: 15rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-64".to_owned(), "top: 16rem;right: 16rem;bottom: 16rem;left: 16rem;".to_owned());/* 16rem = 256px*/
    let _ = top_right_bottom_left.insert("inset-x-64".to_owned(), "left: 16rem;right: 16rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-64".to_owned(), "top: 16rem;bottom: 16rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-64".to_owned(), "top: 16rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-64".to_owned(), "right: 16rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-64".to_owned(), "bottom: 16rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-64".to_owned(), "left: 16rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-72".to_owned(), "top: 18rem;right: 18rem;bottom: 18rem;left: 18rem;".to_owned());/* 18rem = 288px*/
    let _ = top_right_bottom_left.insert("inset-x-72".to_owned(), "left: 18rem;right: 18rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-72".to_owned(), "top: 18rem;bottom: 18rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-72".to_owned(), "top: 18rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-72".to_owned(), "right: 18rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-72".to_owned(), "bottom: 18rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-72".to_owned(), "left: 18rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-80".to_owned(), "top: 20rem;right: 20rem;bottom: 20rem;left: 20rem;".to_owned());/* 20rem = 320px*/
    let _ = top_right_bottom_left.insert("inset-x-80".to_owned(), "left: 20rem;right: 20rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-80".to_owned(), "top: 20rem;bottom: 20rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-80".to_owned(), "top: 20rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-80".to_owned(), "right: 20rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-80".to_owned(), "bottom: 20rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-80".to_owned(), "left: 20rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-96".to_owned(), "top: 24rem;right: 24rem;bottom: 24rem;left: 24rem;".to_owned());/* 24rem = 384px*/
    let _ = top_right_bottom_left.insert("inset-x-96".to_owned(), "left: 24rem;right: 24rem;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-96".to_owned(), "top: 24rem;bottom: 24rem;".to_owned());
    let _ = top_right_bottom_left.insert("top-96".to_owned(), "top: 24rem;".to_owned());
    let _ = top_right_bottom_left.insert("right-96".to_owned(), "right: 24rem;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-96".to_owned(), "bottom: 24rem;".to_owned());
    let _ = top_right_bottom_left.insert("left-96".to_owned(), "left: 24rem;".to_owned());

    let _ = top_right_bottom_left.insert("inset-auto".to_owned(), "top: auto;right: auto;bottom: auto;left: auto;".to_owned());
    let _ = top_right_bottom_left.insert("inset-1/2".to_owned(), "top: 50%;right: 50%;bottom: 50%;left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-1/3".to_owned(), "top: 33.333333%;right: 33.333333%;bottom: 33.333333%;left: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-2/3".to_owned(), "top: 66.666667%;right: 66.666667%;bottom: 66.666667%;left: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-1/4".to_owned(), "top: 25%;right: 25%;bottom: 25%;left: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-2/4".to_owned(), "top: 50%;right: 50%;bottom: 50%;left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-3/4".to_owned(), "top: 75%;right: 75%;bottom: 75%;left: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-full".to_owned(), "top: 100%;right: 100%;bottom: 100%;left: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("inset-x-auto".to_owned(), "right: auto;left: auto;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-1/2".to_owned(), "right: 50%;left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-1/3".to_owned(), "right: 33.333333%;left: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-2/3".to_owned(), "right: 66.666667%;left: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-1/4".to_owned(), "right: 25%;left: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-2/4".to_owned(), "right: 50%;left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-3/4".to_owned(), "right: 75%;left: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-x-full".to_owned(), "right: 100%;left: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("inset-y-auto".to_owned(), "top: auto;bottom: auto;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-1/2".to_owned(), "top: 50%;bottom: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-1/3".to_owned(), "top: 33.333333%;bottom: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-2/3".to_owned(), "top: 66.666667%;bottom: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-1/4".to_owned(), "top: 25%;bottom: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-2/4".to_owned(), "top: 50%;bottom: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-3/4".to_owned(), "top: 75%;bottom: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("inset-y-full".to_owned(), "top: 100%;bottom: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("top-auto".to_owned(), "top: auto;".to_owned());
    let _ = top_right_bottom_left.insert("top-1/2".to_owned(), "top: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("top-1/3".to_owned(), "top: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("top-2/3".to_owned(), "top: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("top-1/4".to_owned(), "top: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("top-2/4".to_owned(), "top: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("top-3/4".to_owned(), "top: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("top-full".to_owned(), "top: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("right-auto".to_owned(), "right: auto;".to_owned());
    let _ = top_right_bottom_left.insert("right-1/2".to_owned(), "right: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("right-1/3".to_owned(), "right: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("right-2/3".to_owned(), "right: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("right-1/4".to_owned(), "right: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("right-2/4".to_owned(), "right: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("right-3/4".to_owned(), "right: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("right-full".to_owned(), "right: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("bottom-auto".to_owned(), "bottom: auto;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-1/2".to_owned(), "bottom: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-1/3".to_owned(), "bottom: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-2/3".to_owned(), "bottom: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-1/4".to_owned(), "bottom: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-2/4".to_owned(), "bottom: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-3/4".to_owned(), "bottom: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("bottom-full".to_owned(), "bottom: 100%;".to_owned());

    let _ = top_right_bottom_left.insert("left-auto".to_owned(), "left: auto;".to_owned());
    let _ = top_right_bottom_left.insert("left-1/2".to_owned(), "left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("left-1/3".to_owned(), "left: 33.333333%;".to_owned());
    let _ = top_right_bottom_left.insert("left-2/3".to_owned(), "left: 66.666667%;".to_owned());
    let _ = top_right_bottom_left.insert("left-1/4".to_owned(), "left: 25%;".to_owned());
    let _ = top_right_bottom_left.insert("left-2/4".to_owned(), "left: 50%;".to_owned());
    let _ = top_right_bottom_left.insert("left-3/4".to_owned(), "left: 75%;".to_owned());
    let _ = top_right_bottom_left.insert("left-full".to_owned(), "left: 100%;".to_owned());

    top_right_bottom_left
}
