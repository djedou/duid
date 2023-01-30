use std::collections::HashMap;


pub(crate) fn display() -> HashMap<String, String> {
    let mut display = HashMap::new();
    let _ = display.insert("block".to_owned(), "display: block;".to_owned());
    let _ = display.insert("inline-block".to_owned(), "display: inline-block;".to_owned());
    let _ = display.insert("inline".to_owned(), "display: inline;".to_owned());
    let _ = display.insert("flex".to_owned(), "display: flex;".to_owned());
    let _ = display.insert("inline-flex".to_owned(), "display: inline-flex;".to_owned());
    let _ = display.insert("table".to_owned(), "display: table;".to_owned());
    let _ = display.insert("inline-table".to_owned(), "display: inline-table;".to_owned());
    let _ = display.insert("table-caption".to_owned(), "display: table-caption;".to_owned());
    let _ = display.insert("table-cell".to_owned(), "display: table-cell;".to_owned());
    let _ = display.insert("table-column".to_owned(), "display: table-column;".to_owned());
    let _ = display.insert("table-column-group".to_owned(), "display: table-column-group;".to_owned());
    let _ = display.insert("table-footer-group".to_owned(), "display: table-footer-group;".to_owned());
    let _ = display.insert("table-header-group".to_owned(), "display: table-header-group;".to_owned());
    let _ = display.insert("table-row-group".to_owned(), "display: table-row-group;".to_owned());
    let _ = display.insert("table-row".to_owned(), "display: table-row;".to_owned());
    let _ = display.insert("flow-root".to_owned(), "display: flow-root;".to_owned());
    let _ = display.insert("grid".to_owned(), "display: grid;".to_owned());
    let _ = display.insert("inline-grid".to_owned(), "display: inline-grid;".to_owned());
    let _ = display.insert("contents".to_owned(), "display: contents;".to_owned());
    let _ = display.insert("list-item".to_owned(), "display: list-item;".to_owned());
    let _ = display.insert("hidden".to_owned(), "display: none;".to_owned());
    
    display
}