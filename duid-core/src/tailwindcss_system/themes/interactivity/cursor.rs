use std::collections::HashMap;


pub(crate) fn cursor() -> HashMap<String, String> {
    let mut cursor = HashMap::new();
    let _ = cursor.insert("cursor-auto".to_owned(), "cursor: auto;".to_owned());
    let _ = cursor.insert("cursor-default".to_owned(), "cursor: default;".to_owned());
    let _ = cursor.insert("cursor-pointer".to_owned(), "cursor: pointer;".to_owned());
    let _ = cursor.insert("cursor-wait".to_owned(), "cursor: wait;".to_owned());
    let _ = cursor.insert("cursor-text".to_owned(), "cursor: text;".to_owned());
    let _ = cursor.insert("cursor-move".to_owned(), "cursor: move;".to_owned());
    let _ = cursor.insert("cursor-help".to_owned(), "cursor: help;".to_owned());
    let _ = cursor.insert("cursor-not-allowed".to_owned(), "cursor: not-allowed;".to_owned());
    let _ = cursor.insert("cursor-none".to_owned(), "cursor: none;".to_owned());
    let _ = cursor.insert("cursor-context-men".to_owned(), "cursor: context-menu;".to_owned());
    let _ = cursor.insert("cursor-progress".to_owned(), "cursor: progress;".to_owned());
    let _ = cursor.insert("cursor-cell".to_owned(), "cursor: cell;".to_owned());
    let _ = cursor.insert("cursor-crosshair".to_owned(), "cursor: crosshair;".to_owned());
    let _ = cursor.insert("cursor-vertical-text".to_owned(), "cursor: vertical-text;".to_owned());
    let _ = cursor.insert("cursor-alias".to_owned(), "cursor: alias;".to_owned());
    let _ = cursor.insert("cursor-copy".to_owned(), "cursor: copy;".to_owned());
    let _ = cursor.insert("cursor-no-drop".to_owned(), "cursor: no-drop;".to_owned());
    let _ = cursor.insert("cursor-grab".to_owned(), "cursor: grab;".to_owned());
    let _ = cursor.insert("cursor-grabbing".to_owned(), "cursor: grabbing;".to_owned());
    let _ = cursor.insert("cursor-all-scroll".to_owned(), "cursor: all-scroll;".to_owned());
    let _ = cursor.insert("cursor-col-resize".to_owned(), "cursor: col-resize;".to_owned());
    let _ = cursor.insert("cursor-row-resize".to_owned(), "cursor: row-resize;".to_owned());
    let _ = cursor.insert("cursor-n-resize".to_owned(), "cursor: n-resize;".to_owned());
    let _ = cursor.insert("cursor-e-resize".to_owned(), "cursor: e-resize;".to_owned());
    let _ = cursor.insert("cursor-s-resize".to_owned(), "cursor: s-resize;".to_owned());
    let _ = cursor.insert("cursor-w-resize".to_owned(), "cursor: w-resize;".to_owned());
    let _ = cursor.insert("cursor-ne-resize".to_owned(), "cursor: ne-resize;".to_owned());
    let _ = cursor.insert("cursor-nw-resize".to_owned(), "cursor: nw-resize;".to_owned());
    let _ = cursor.insert("cursor-se-resize".to_owned(), "cursor: se-resize;".to_owned());
    let _ = cursor.insert("cursor-sw-resize".to_owned(), "cursor: sw-resize;".to_owned());
    let _ = cursor.insert("cursor-ew-resize".to_owned(), "cursor: ew-resize;".to_owned());
    let _ = cursor.insert("cursor-ns-resize".to_owned(), "cursor: ns-resize;".to_owned());
    let _ = cursor.insert("cursor-nesw-resize".to_owned(), "cursor: nesw-resize;".to_owned());
    let _ = cursor.insert("cursor-nwse-resize".to_owned(), "cursor: nwse-resize;".to_owned());
    let _ = cursor.insert("cursor-zoom-in".to_owned(), "cursor: zoom-in;".to_owned());
    let _ = cursor.insert("cursor-zoom-out".to_owned(), "cursor: zoom-out;".to_owned());
    let _ = cursor.insert("cursor-inherit".to_owned(), "cursor: inherit;".to_owned());

    cursor
}