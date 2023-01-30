use std::collections::HashMap;


pub(crate) fn grid_auto_flow() -> HashMap<String, String> {
    let mut grid_auto_flow = HashMap::new();
    let _ = grid_auto_flow.insert("grid-flow-row".to_owned(), "grid-auto-flow: row;".to_owned());
    let _ = grid_auto_flow.insert("grid-flow-col".to_owned(), "grid-auto-flow: column;".to_owned());
    let _ = grid_auto_flow.insert("grid-flow-dense".to_owned(), "grid-auto-flow: dense;".to_owned());
    let _ = grid_auto_flow.insert("grid-flow-row-dense".to_owned(), "grid-auto-flow: row dense;".to_owned());
    let _ = grid_auto_flow.insert("grid-flow-col-dense".to_owned(), "grid-auto-flow: column dense;".to_owned());

    grid_auto_flow
}