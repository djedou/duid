use std::collections::HashMap;


pub(crate) fn font_family() -> HashMap<String, String> {
    let mut font_family = HashMap::new();
    let _ = font_family.insert("font-sans".to_owned(), "font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, \"Noto Sans\", sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\";".to_owned());
    let _ = font_family.insert("font-serif".to_owned(), "font-family: ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif;".to_owned());
    let _ = font_family.insert("font-mono".to_owned(), "font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace;".to_owned());
    font_family
}