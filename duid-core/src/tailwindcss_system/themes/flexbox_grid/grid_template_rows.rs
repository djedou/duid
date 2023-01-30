use std::collections::HashMap;


pub(crate) fn grid_template_rows() -> HashMap<String, String> {
    let mut grid_template_rows = HashMap::new();
    let _ = grid_template_rows.insert("grid-rows-1".to_owned(), "grid-template-rows: repeat(1, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-2".to_owned(), "grid-template-rows: repeat(2, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-3".to_owned(), "grid-template-rows: repeat(3, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-4".to_owned(), "grid-template-rows: repeat(4, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-5".to_owned(), "grid-template-rows: repeat(5, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-6".to_owned(), "grid-template-rows: repeat(6, minmax(0, 1fr));".to_owned());
    let _ = grid_template_rows.insert("grid-rows-none".to_owned(), "grid-template-rows: none;".to_owned());

    grid_template_rows
}
