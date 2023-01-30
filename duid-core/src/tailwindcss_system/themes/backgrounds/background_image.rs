use std::collections::HashMap;

// usage: combine **background_image** with **gradient_color_stops**  
// ex: bg-gradient-to-t from-green-600 via-violet-600, bg-gradient-to-t from-green-600
pub(crate) fn background_image() -> HashMap<String, String> {
    let mut background_image = HashMap::new();
    let _ = background_image.insert("bg-none".to_owned(), "background-image: none;".to_owned());
    let _ = background_image.insert("bg-".to_owned(), "background-image: url;".to_owned());
    let _ = background_image.insert("bg-gradient-to-t".to_owned(), "background-image: linear-gradient(to top, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-tr".to_owned(), "background-image: linear-gradient(to top right, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-r".to_owned(), "background-image: linear-gradient(to right, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-br".to_owned(), "background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-b".to_owned(), "background-image: linear-gradient(to bottom, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-bl".to_owned(), "background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-l".to_owned(), "background-image: linear-gradient(to left, var(--tw-gradient-stops));".to_owned());
    let _ = background_image.insert("bg-gradient-to-tl".to_owned(), "background-image: linear-gradient(to top left, var(--tw-gradient-stops));".to_owned());
    

    background_image
}
