use std::collections::HashMap;


pub(crate) fn font_variant_numeric() -> HashMap<String, String> {
    let mut font_variant_numeric = HashMap::new();
    let _ = font_variant_numeric.insert("normal-nums".to_owned(), "font-variant-numeric: normal;".to_owned());
    let _ = font_variant_numeric.insert("ordinal".to_owned(), "font-variant-numeric: ordinal;".to_owned());
    let _ = font_variant_numeric.insert("slashed-zero".to_owned(), "font-variant-numeric: slashed-zero;".to_owned());
    let _ = font_variant_numeric.insert("lining-nums".to_owned(), "font-variant-numeric: lining-nums;".to_owned());
    let _ = font_variant_numeric.insert("oldstyle-nums".to_owned(), "font-variant-numeric: oldstyle-nums;".to_owned());
    let _ = font_variant_numeric.insert("proportional-nums".to_owned(), "font-variant-numeric: proportional-nums;".to_owned());
    let _ = font_variant_numeric.insert("tabular-nums".to_owned(), "font-variant-numeric: tabular-nums;".to_owned());
    let _ = font_variant_numeric.insert("diagonal-fractions".to_owned(), "font-variant-numeric: diagonal-fractions;".to_owned());
    let _ = font_variant_numeric.insert("stacked-fractions".to_owned(), "font-variant-numeric: stacked-fractions;".to_owned());

    font_variant_numeric
}