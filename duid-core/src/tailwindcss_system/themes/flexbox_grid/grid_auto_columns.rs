use std::collections::HashMap;


pub(crate) fn grid_auto_columns() -> HashMap<String, String> {
    let mut grid_auto_columns = HashMap::new();
    let _ = grid_auto_columns.insert("auto-cols-auto".to_owned(), "grid-auto-columns: auto;".to_owned());
    let _ = grid_auto_columns.insert("auto-cols-min".to_owned(), "grid-auto-columns: min-content;".to_owned());
    let _ = grid_auto_columns.insert("auto-cols-max".to_owned(), "grid-auto-columns: max-content;".to_owned());
    let _ = grid_auto_columns.insert("auto-cols-fr".to_owned(), "grid-auto-columns: minmax(0, 1fr);".to_owned());
    

    grid_auto_columns
}