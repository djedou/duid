use std::collections::HashMap;


pub(crate) fn order() -> HashMap<String, String> {
    let mut order = HashMap::new();
    let _ = order.insert("order-1".to_owned(), "order: 1;".to_owned());
    let _ = order.insert("order-2".to_owned(), "order: 2;".to_owned());
    let _ = order.insert("order-3".to_owned(), "order: 3;".to_owned());
    let _ = order.insert("order-4".to_owned(), "order: 4;".to_owned());
    let _ = order.insert("order-5".to_owned(), "order: 5;".to_owned());
    let _ = order.insert("order-6".to_owned(), "order: 6;".to_owned());
    let _ = order.insert("order-7".to_owned(), "order: 7;".to_owned());
    let _ = order.insert("order-8".to_owned(), "order: 8;".to_owned());
    let _ = order.insert("order-9".to_owned(), "order: 9;".to_owned());
    let _ = order.insert("order-10".to_owned(), "order: 10;".to_owned());
    let _ = order.insert("order-11".to_owned(), "order: 11;".to_owned());
    let _ = order.insert("order-12".to_owned(), "order: 12;".to_owned());
    let _ = order.insert("order-first".to_owned(), "order: -9999;".to_owned());
    let _ = order.insert("order-last".to_owned(), "order: 9999;".to_owned());
    let _ = order.insert("order-none".to_owned(), "order: 0;".to_owned());

    order
}