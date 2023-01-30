use std::collections::HashMap;


pub(crate) fn table_layout() -> HashMap<String, String> {
    let mut table_layout = HashMap::new();
    let _ = table_layout.insert("table-auto".to_owned(), "table-layout: auto;".to_owned());
    let _ = table_layout.insert("table-fixed".to_owned(), "table-layout: fixed;".to_owned());

    table_layout
}