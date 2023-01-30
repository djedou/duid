use std::collections::HashMap;


pub(crate) fn grid_auto_rows() -> HashMap<String, String> {
    let mut grid_auto_rows = HashMap::new();
    let _ = grid_auto_rows.insert("auto-rows-auto".to_owned(), "grid-auto-rows: auto;".to_owned());
    let _ = grid_auto_rows.insert("auto-rows-min".to_owned(), "grid-auto-rows: min-content;".to_owned());
    let _ = grid_auto_rows.insert("auto-rows-max".to_owned(), "grid-auto-rows: max-content;".to_owned());
    let _ = grid_auto_rows.insert("auto-rows-fr".to_owned(), "grid-auto-rows: minmax(0, 1fr);".to_owned());

    grid_auto_rows
}