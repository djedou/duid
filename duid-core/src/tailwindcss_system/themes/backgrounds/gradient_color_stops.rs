use std::collections::HashMap;


pub(crate) fn gradient_color_stops() -> HashMap<String, String> {
    let mut gradient_color_stops = HashMap::new();
    let _ = gradient_color_stops.insert("gradient-color-from-var".to_owned(), "--duid-gradient-color-from".to_owned());
    let _ = gradient_color_stops.insert("gradient-color-via-var".to_owned(), "--duid-gradient-color-via".to_owned());
    let _ = gradient_color_stops.insert("gradient-color-to-var".to_owned(), "--duid-gradient-color-to".to_owned());
    
    let _ = gradient_color_stops.insert("gradient-from-to-stops".to_owned(), "--duid-gradient-color-stops: var(--duid-gradient-color-from), var(--duid-gradient-color-to);".to_owned());
    let _ = gradient_color_stops.insert("gradient-via-stops".to_owned(), "--duid-gradient-color-stops: var(--duid-gradient-color-from), var(--duid-gradient-color-via), var(--duid-gradient-color-to);".to_owned());
    
    gradient_color_stops
}