use std::collections::HashMap;


pub(crate) fn background_blend_mode() -> HashMap<String, String> {
    let mut background_blend_mode = HashMap::new();
    let _ = background_blend_mode.insert("bg-blend-normal".to_owned(), "background-blend-mode: normal;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-multiply".to_owned(), "background-blend-mode: multiply;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-screen".to_owned(), "background-blend-mode: screen;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-overlay".to_owned(), "background-blend-mode: overlay;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-darken".to_owned(), "background-blend-mode: darken;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-lighten".to_owned(), "background-blend-mode: lighten;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-color-dodge".to_owned(), "background-blend-mode: color-dodge;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-color-burn".to_owned(), "background-blend-mode: color-burn;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-hard-light".to_owned(), "background-blend-mode: hard-light;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-soft-light".to_owned(), "background-blend-mode: soft-light;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-difference".to_owned(), "background-blend-mode: difference;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-exclusion".to_owned(), "background-blend-mode: exclusion;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-hue".to_owned(), "background-blend-mode: hue;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-saturation".to_owned(), "background-blend-mode: saturation;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-color".to_owned(), "background-blend-mode: color;".to_owned());
    let _ = background_blend_mode.insert("bg-blend-luminosity".to_owned(), "background-blend-mode: luminosity;".to_owned());

    background_blend_mode
}

/*

*/