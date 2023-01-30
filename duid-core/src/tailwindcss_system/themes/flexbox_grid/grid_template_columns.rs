use std::collections::HashMap;


pub(crate) fn grid_template_columns() -> HashMap<String, String> {
    let mut grid_template_columns = HashMap::new();
    let _ = grid_template_columns.insert("grid-cols-1".to_owned(), "grid-template-columns: repeat(1, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-2".to_owned(), "grid-template-columns: repeat(2, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-3".to_owned(), "grid-template-columns: repeat(3, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-4".to_owned(), "grid-template-columns: repeat(4, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-5".to_owned(), "grid-template-columns: repeat(5, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-6".to_owned(), "grid-template-columns: repeat(6, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-7".to_owned(), "grid-template-columns: repeat(7, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-8".to_owned(), "grid-template-columns: repeat(8, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-9".to_owned(), "grid-template-columns: repeat(9, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-10".to_owned(), "grid-template-columns: repeat(10, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-11".to_owned(), "grid-template-columns: repeat(11, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-12".to_owned(), "grid-template-columns: repeat(12, minmax(0, 1fr));".to_owned());
    let _ = grid_template_columns.insert("grid-cols-none".to_owned(), "grid-template-columns: none;".to_owned());

    grid_template_columns
}