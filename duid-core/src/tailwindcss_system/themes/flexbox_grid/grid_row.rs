use std::collections::HashMap;


pub(crate) fn grid_row() -> HashMap<String, String> {
    let mut grid_row = HashMap::new();
    let _ = grid_row.insert("row-auto".to_owned(), "grid-row: auto;".to_owned());
    let _ = grid_row.insert("row-span-1".to_owned(), "grid-row: span 1 / span 1;".to_owned());
    let _ = grid_row.insert("row-span-2".to_owned(), "grid-row: span 2 / span 2;".to_owned());
    let _ = grid_row.insert("row-span-3".to_owned(), "grid-row: span 3 / span 3;".to_owned());
    let _ = grid_row.insert("row-span-4".to_owned(), "grid-row: span 4 / span 4;".to_owned());
    let _ = grid_row.insert("row-span-5".to_owned(), "grid-row: span 5 / span 5;".to_owned());
    let _ = grid_row.insert("row-span-6".to_owned(), "grid-row: span 6 / span 6;".to_owned());
    let _ = grid_row.insert("row-span-full".to_owned(), "grid-row: 1 / -1;".to_owned());
    let _ = grid_row.insert("row-start-1".to_owned(), "grid-row-start: 1;".to_owned());
    let _ = grid_row.insert("row-start-2".to_owned(), "grid-row-start: 2;".to_owned());
    let _ = grid_row.insert("row-start-3".to_owned(), "grid-row-start: 3;".to_owned());
    let _ = grid_row.insert("row-start-4".to_owned(), "grid-row-start: 4;".to_owned());
    let _ = grid_row.insert("row-start-5".to_owned(), "grid-row-start: 5;".to_owned());
    let _ = grid_row.insert("row-start-6".to_owned(), "grid-row-start: 6;".to_owned());
    let _ = grid_row.insert("row-start-7".to_owned(), "grid-row-start: 7;".to_owned());
    let _ = grid_row.insert("row-start-auto".to_owned(), "grid-row-start: auto;".to_owned());
    let _ = grid_row.insert("row-end-1".to_owned(), "grid-row-end: 1;".to_owned());
    let _ = grid_row.insert("row-end-2".to_owned(), "grid-row-end: 2;".to_owned());
    let _ = grid_row.insert("row-end-3".to_owned(), "grid-row-end: 3;".to_owned());
    let _ = grid_row.insert("row-end-4".to_owned(), "grid-row-end: 4;".to_owned());
    let _ = grid_row.insert("row-end-5".to_owned(), "grid-row-end: 5;".to_owned());
    let _ = grid_row.insert("row-end-6".to_owned(), "grid-row-end: 6;".to_owned());
    let _ = grid_row.insert("row-end-7".to_owned(), "grid-row-end: 7;".to_owned());
    let _ = grid_row.insert("row-end-auto".to_owned(), "grid-row-end: auto;".to_owned());

    grid_row
}