use std::collections::HashMap;


pub(crate) fn background_repeat() -> HashMap<String, String> {
    let mut background_repeat = HashMap::new();
    let _ = background_repeat.insert("bg-repeat".to_owned(), "background-repeat: repeat;".to_owned());
    let _ = background_repeat.insert("bg-no-repeat".to_owned(), "background-repeat: no-repeat;".to_owned());
    let _ = background_repeat.insert("bg-repeat-x".to_owned(), "background-repeat: repeat-x;".to_owned());
    let _ = background_repeat.insert("bg-repeat-y".to_owned(), "background-repeat: repeat-y;".to_owned());
    let _ = background_repeat.insert("bg-repeat-round".to_owned(), "background-repeat: round;".to_owned());
    let _ = background_repeat.insert("bg-repeat-space".to_owned(), "background-repeat: space;".to_owned());

    background_repeat
}