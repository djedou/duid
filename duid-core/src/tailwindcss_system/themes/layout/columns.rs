use std::collections::HashMap;


pub(crate) fn columns() -> HashMap<String, String> {
    let mut columns = HashMap::new();
    let _ = columns.insert("columns-1".to_owned(), "columns: 1;".to_owned());
    let _ = columns.insert("columns-2".to_owned(), "columns: 2;".to_owned());
    let _ = columns.insert("columns-3".to_owned(), "columns: 3;".to_owned());
    let _ = columns.insert("columns-4".to_owned(), "columns: 4;".to_owned());
    let _ = columns.insert("columns-5".to_owned(), "columns: 5;".to_owned());
    let _ = columns.insert("columns-6".to_owned(), "columns: 6;".to_owned());
    let _ = columns.insert("columns-7".to_owned(), "columns: 7;".to_owned());
    let _ = columns.insert("columns-8".to_owned(), "columns: 8;".to_owned());
    let _ = columns.insert("columns-9".to_owned(), "columns: 9;".to_owned());
    let _ = columns.insert("columns-10".to_owned(), "columns: 10;".to_owned());
    let _ = columns.insert("columns-11".to_owned(), "columns: 11;".to_owned());
    let _ = columns.insert("columns-12".to_owned(), "columns: 12;".to_owned());
    let _ = columns.insert("columns-auto".to_owned(), "columns: auto;".to_owned());
    let _ = columns.insert("columns-3xs".to_owned(), "columns: 16rem;".to_owned()); /* 256px*/
    let _ = columns.insert("columns-2xs".to_owned(), "columns: 18rem;".to_owned()); /* 288px*/
    let _ = columns.insert("columns-xs".to_owned(), "columns: 20rem;".to_owned()); /* 320px*/
    let _ = columns.insert("columns-sm".to_owned(), "columns: 24rem;".to_owned()); /* 384px*/
    let _ = columns.insert("columns-md".to_owned(), "columns: 28rem;".to_owned()); /* 448px*/
    let _ = columns.insert("columns-lg".to_owned(), "columns: 32rem;".to_owned()); /* 512px*/
    let _ = columns.insert("columns-xl".to_owned(), "columns: 36rem;".to_owned()); /* 576px*/
    let _ = columns.insert("columns-2xl".to_owned(), "columns: 42rem;".to_owned()); /* 672px*/
    let _ = columns.insert("columns-3xl".to_owned(), "columns: 48rem;".to_owned()); /* 768px*/
    let _ = columns.insert("columns-4xl".to_owned(), "columns: 56rem;".to_owned()); /* 896px*/
    let _ = columns.insert("columns-5xl".to_owned(), "columns: 64rem;".to_owned()); /* 1024px*/
    let _ = columns.insert("columns-6xl".to_owned(), "columns: 72rem;".to_owned()); /* 1152px*/
    let _ = columns.insert("columns-7xl".to_owned(), "columns: 80rem;".to_owned()); /* 1280px*/

    columns
}
