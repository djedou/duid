use std::collections::HashMap;


pub(crate) fn mix_blend_mode() -> HashMap<String, String> {
    let mut mix_blend_mode = HashMap::new();
    let _ = mix_blend_mode.insert("mix-blend-normal".to_owned(), "mix-blend-mode: normal;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-multiply".to_owned(), "mix-blend-mode: multiply;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-screen".to_owned(), "mix-blend-mode: screen;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-overlay".to_owned(), "mix-blend-mode: overlay;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-darken".to_owned(), "mix-blend-mode: darken;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-lighten".to_owned(), "mix-blend-mode: lighten;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-color-dodge".to_owned(), "mix-blend-mode: color-dodge;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-color-burn".to_owned(), "mix-blend-mode: color-burn;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-hard-light".to_owned(), "mix-blend-mode: hard-light;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-soft-light".to_owned(), "mix-blend-mode: soft-light;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-difference".to_owned(), "mix-blend-mode: difference;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-exclusion".to_owned(), "mix-blend-mode: exclusion;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-hue".to_owned(), "mix-blend-mode: hue;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-saturation".to_owned(), "mix-blend-mode: saturation;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-color".to_owned(), "mix-blend-mode: color;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-luminosity".to_owned(), "mix-blend-mode: luminosity;".to_owned());
    let _ = mix_blend_mode.insert("mix-blend-plus-lighter".to_owned(), "mix-blend-mode: plus-lighter;".to_owned());

    mix_blend_mode
}