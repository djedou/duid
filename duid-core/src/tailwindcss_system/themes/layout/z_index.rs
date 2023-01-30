use std::collections::HashMap;


pub(crate) fn z_index() -> HashMap<String, String> {
    let mut z_index = HashMap::new();
    let _ = z_index.insert("z-0".to_owned(), "z-index: 0;".to_owned());
    let _ = z_index.insert("z-1".to_owned(), "z-index: 1;".to_owned());
    let _ = z_index.insert("z-2".to_owned(), "z-index: 2;".to_owned());
    let _ = z_index.insert("z-3".to_owned(), "z-index: 3;".to_owned());
    let _ = z_index.insert("z-4".to_owned(), "z-index: 4;".to_owned());
    let _ = z_index.insert("z-5".to_owned(), "z-index: 5;".to_owned());
    let _ = z_index.insert("z-6".to_owned(), "z-index: 6;".to_owned());
    let _ = z_index.insert("z-7".to_owned(), "z-index: 7;".to_owned());
    let _ = z_index.insert("z-8".to_owned(), "z-index: 8;".to_owned());
    let _ = z_index.insert("z-9".to_owned(), "z-index: 9;".to_owned());
    let _ = z_index.insert("z-10".to_owned(), "z-index: 10;".to_owned());
    let _ = z_index.insert("z-98".to_owned(), "z-index: 98;".to_owned());
    let _ = z_index.insert("z-99".to_owned(), "z-index: 99;".to_owned());
    let _ = z_index.insert("z-100".to_owned(), "z-index: 100;".to_owned());
    let _ = z_index.insert("z-999".to_owned(), "z-index: 999;".to_owned());
    let _ = z_index.insert("z-9999".to_owned(), "z-index: 9999;".to_owned());
    let _ = z_index.insert("z-99999".to_owned(), "z-index: 99999;".to_owned());
    let _ = z_index.insert("z-999999".to_owned(), "z-index: 999999;".to_owned());
    let _ = z_index.insert("z-9999999".to_owned(), "z-index: 9999999;".to_owned());
    let _ = z_index.insert("z-max".to_owned(), "z-index: 2147483647;".to_owned());
    let _ = z_index.insert("z-inherit".to_owned(), "z-index: inherit;".to_owned());
    let _ = z_index.insert("z-initial".to_owned(), "z-index: initial;".to_owned());
    let _ = z_index.insert("z-unset".to_owned(), "z-index: unset;".to_owned());
    let _ = z_index.insert("z-auto".to_owned(), "z-index: auto;".to_owned());

    z_index
}
