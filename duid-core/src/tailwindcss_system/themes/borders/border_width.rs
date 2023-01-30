
use std::collections::HashMap;


pub(crate) fn border_width() -> HashMap<String, String> {
    let mut border_width = HashMap::new();
    let _ = border_width.insert("border-0".to_owned(), "border-width: 0px;".to_owned());
    let _ = border_width.insert("border-2".to_owned(), "border-width: 2px;".to_owned());
    let _ = border_width.insert("border-4".to_owned(), "border-width: 4px;".to_owned());
    let _ = border_width.insert("border-8".to_owned(), "border-width: 8px;".to_owned());
    let _ = border_width.insert("border".to_owned(), "border-width: 1px;".to_owned());
    let _ = border_width.insert("border-x-0".to_owned(), "border-left-width: 0px;border-right-width: 0px;".to_owned());
    let _ = border_width.insert("border-x-2".to_owned(), "border-left-width: 2px;border-right-width: 2px;".to_owned());
    let _ = border_width.insert("border-x-4".to_owned(), "border-left-width: 4px;border-right-width: 4px;".to_owned());
    let _ = border_width.insert("border-x-8".to_owned(), "border-left-width: 8px;border-right-width: 8px;".to_owned());
    let _ = border_width.insert("border-x".to_owned(), "border-left-width: 1px;border-right-width: 1px;".to_owned());
    let _ = border_width.insert("border-y-0".to_owned(), "border-top-width: 0px;border-bottom-width: 0px;".to_owned());
    let _ = border_width.insert("border-y-2".to_owned(), "border-top-width: 2px;border-bottom-width: 2px;".to_owned());
    let _ = border_width.insert("border-y-4".to_owned(), "border-top-width: 4px;border-bottom-width: 4px;".to_owned());
    let _ = border_width.insert("border-y-8".to_owned(), "border-top-width: 8px;border-bottom-width: 8px;".to_owned());
    let _ = border_width.insert("border-y".to_owned(), "border-top-width: 1px;border-bottom-width: 1px;".to_owned());
    let _ = border_width.insert("border-t-0".to_owned(), "border-top-width: 0px;".to_owned());
    let _ = border_width.insert("border-t-2".to_owned(), "border-top-width: 2px;".to_owned());
    let _ = border_width.insert("border-t-4".to_owned(), "border-top-width: 4px;".to_owned());
    let _ = border_width.insert("border-t-8".to_owned(), "border-top-width: 8px;".to_owned());
    let _ = border_width.insert("border-t".to_owned(), "border-top-width: 1px;".to_owned());
    let _ = border_width.insert("border-r-0".to_owned(), "border-right-width: 0px;".to_owned());
    let _ = border_width.insert("border-r-2".to_owned(), "border-right-width: 2px;".to_owned());
    let _ = border_width.insert("border-r-4".to_owned(), "border-right-width: 4px;".to_owned());
    let _ = border_width.insert("border-r-8".to_owned(), "border-right-width: 8px;".to_owned());
    let _ = border_width.insert("border-r".to_owned(), "border-right-width: 1px;".to_owned());
    let _ = border_width.insert("border-b-0".to_owned(), "border-bottom-width: 0px;".to_owned());
    let _ = border_width.insert("border-b-2".to_owned(), "border-bottom-width: 2px;".to_owned());
    let _ = border_width.insert("border-b-4".to_owned(), "border-bottom-width: 4px;".to_owned());
    let _ = border_width.insert("border-b-8".to_owned(), "border-bottom-width: 8px;".to_owned());
    let _ = border_width.insert("border-b".to_owned(), "border-bottom-width: 1px;".to_owned());
    let _ = border_width.insert("border-l-0".to_owned(), "border-left-width: 0px;".to_owned());
    let _ = border_width.insert("border-l-2".to_owned(), "border-left-width: 2px;".to_owned());
    let _ = border_width.insert("border-l-4".to_owned(), "border-left-width: 4px;".to_owned());
    let _ = border_width.insert("border-l-8".to_owned(), "border-left-width: 8px;".to_owned());
    let _ = border_width.insert("border-l".to_owned(), "border-left-width: 1px;".to_owned());

    border_width
}