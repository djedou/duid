
use std::collections::HashMap;


pub(crate) fn divide_width() -> HashMap<String, String> {
    let mut divide_width = HashMap::new();
    let _ = divide_width.insert("divide-x-0".to_owned(), "border-right-width: 0px;border-left-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-x-2".to_owned(), "border-right-width: 0px;border-left-width: 2px;".to_owned());
    let _ = divide_width.insert("divide-x-4".to_owned(), "border-right-width: 0px;border-left-width: 4px;".to_owned());
    let _ = divide_width.insert("divide-x-8".to_owned(), "border-right-width: 0px;border-left-width: 8px;".to_owned());
    let _ = divide_width.insert("divide-x".to_owned(), "border-right-width: 0px;border-left-width: 1px;".to_owned());
    let _ = divide_width.insert("divide-y-0".to_owned(), "border-top-width: 0px;border-bottom-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-y-2".to_owned(), "border-top-width: 2px;border-bottom-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-y-4".to_owned(), "border-top-width: 4px;border-bottom-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-y-8".to_owned(), "border-top-width: 8px;border-bottom-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-y".to_owned(), "border-top-width: 1px;border-bottom-width: 0px;".to_owned());
    let _ = divide_width.insert("divide-y-reverse-var".to_owned(), "--duid-divide-y-reverse: 1;".to_owned());
    let _ = divide_width.insert("divide-x-reverse-var".to_owned(), "--duid-divide-x-reverse: 1;".to_owned());

    divide_width
}