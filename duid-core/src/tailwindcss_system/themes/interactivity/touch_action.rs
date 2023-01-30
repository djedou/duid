use std::collections::HashMap;


pub(crate) fn touch_action() -> HashMap<String, String> {
    let mut touch_action = HashMap::new();
    let _ = touch_action.insert("touch-auto".to_owned(), "touch-action: auto;".to_owned());
    let _ = touch_action.insert("touch-none".to_owned(), "touch-action: none;".to_owned());
    let _ = touch_action.insert("touch-pan-x".to_owned(), "touch-action: pan-x;".to_owned());
    let _ = touch_action.insert("touch-pan-left".to_owned(), "touch-action: pan-left;".to_owned());
    let _ = touch_action.insert("touch-pan-right".to_owned(), "touch-action: pan-right;".to_owned());
    let _ = touch_action.insert("touch-pan-y".to_owned(), "touch-action: pan-y;".to_owned());
    let _ = touch_action.insert("touch-pan-up".to_owned(), "touch-action: pan-up;".to_owned());
    let _ = touch_action.insert("touch-pan-down".to_owned(), "touch-action: pan-down;".to_owned());
    let _ = touch_action.insert("touch-pinch-zoom".to_owned(), "touch-action: pinch-zoom;".to_owned());
    let _ = touch_action.insert("touch-manipulation".to_owned(), "touch-action: manipulation;".to_owned());

    touch_action
}