mod colors;
mod layout;
mod base;
mod spacing;
mod flexbox_grid;
mod sizing;
mod typography;
mod backgrounds;
mod borders;
mod effects;
mod filters;
mod tables;
mod transitions_and_animation;
mod transforms;
mod interactivity;
mod svg;

use std::collections::HashMap;
use colors::*;
use layout::*;
use base::*;
use spacing::*;
use flexbox_grid::*;
use sizing::*;
use typography::*;
use backgrounds::*;
use borders::*;
use effects::*;
use filters::*;
use tables::*;
use transitions_and_animation::*;
use transforms::*;
use interactivity::*;
use svg::*;


/// Theme
#[derive(Debug, Clone)]
pub struct TailwindTheme {
    pub mode: ThemeMode,
    pub themes: HashMap<String, String>,
    pub variables: HashMap<String, String>,
    pub mode_variables: HashMap<String, String>,
}


/// Theme mode.
#[derive(Debug, Clone)]
pub enum ThemeMode {
    Dark,
    Light
}



impl TailwindTheme {
    pub fn new(mode: ThemeMode) -> TailwindTheme {
        
        let tailwind = TailwindTheme {
            mode: mode.clone(),
            themes: TailwindTheme::set_themes(),
            variables: TailwindTheme::set_variables(),
            mode_variables: HashMap::with_capacity(0)
        };

        tailwind
    }

    fn set_variables() -> HashMap<String, String> {
        let mut variables = HashMap::with_capacity(0);
        let _ = variables.insert("duid-base-styles".to_owned(), base_styles());

        variables
    }
    
    fn set_themes() -> HashMap<String, String> {
        let mut themes = HashMap::with_capacity(0);
        // colors
        themes.extend(colors_styles());
        themes.extend(background_colors_styles());
        themes.extend(border_colors_styles());
        themes.extend(text_decoration_colors_styles());
        themes.extend(divide_colors_styles());
        themes.extend(outline_colors_styles());
        themes.extend(ring_colors_styles());
        themes.extend(ring_offset_colors_styles());
        themes.extend(box_shadow_colors_styles());
        themes.extend(accent_colors_styles());
        themes.extend(caret_colors_styles());
        // layouts
        themes.extend(aspect_ratio());
        themes.extend(box_decoration_break());
        themes.extend(box_sizing());
        themes.extend(break_after());
        themes.extend(break_before());
        themes.extend(break_inside());
        themes.extend(clear());
        themes.extend(columns());
        themes.extend(container());
        themes.extend(display());
        themes.extend(floats());
        themes.extend(isolation());
        themes.extend(object_fit());
        themes.extend(object_position());
        themes.extend(overflow());
        themes.extend(overscroll_behavior());
        themes.extend(position());
        themes.extend(top_right_bottom_left());
        themes.extend(visibility());
        themes.extend(z_index());
        // spacing
        themes.extend(margin());
        themes.extend(padding());
        // flexbox and grid
        themes.extend(align_content());
        themes.extend(align_items());
        themes.extend(align_self());
        themes.extend(flex_basis());
        themes.extend(flex_direction());
        themes.extend(flex_grow());
        themes.extend(flex_shrink());
        themes.extend(flex_wrap());
        themes.extend(flex());
        themes.extend(gap());
        themes.extend(grid_auto_columns());
        themes.extend(grid_auto_flow());
        themes.extend(grid_auto_rows());
        themes.extend(grid_column());
        themes.extend(grid_row());
        themes.extend(grid_template_columns());
        themes.extend(grid_template_rows());
        themes.extend(justify_content());
        themes.extend(justify_items());
        themes.extend(justify_self());
        themes.extend(order());
        themes.extend(place_content());
        themes.extend(place_items());
        themes.extend(place_self());
        // sizing
        themes.extend(height());
        themes.extend(max_height());
        themes.extend(max_width());
        themes.extend(min_height());
        themes.extend(min_width());
        themes.extend(width());
        // typography
        themes.extend(content());
        themes.extend(font_family());
        themes.extend(font_size());
        themes.extend(font_smoothing());
        themes.extend(font_style());
        themes.extend(font_variant_numeric());
        themes.extend(font_weight());
        themes.extend(letter_spacing());
        themes.extend(line_height());
        themes.extend(list_style_position());
        themes.extend(list_style_type());
        themes.extend(text_align());
        themes.extend(text_decoration_style());
        themes.extend(text_decoration_thickness());
        themes.extend(text_decoration());
        themes.extend(text_indent());
        themes.extend(text_overflow());
        themes.extend(text_transform());
        themes.extend(text_underline_offset());
        themes.extend(vertical_align());
        themes.extend(white_space());
        themes.extend(word_break());
        // backgrounds
        themes.extend(background_attachment());
        themes.extend(background_clip());
        themes.extend(background_origin());
        themes.extend(background_position());
        themes.extend(background_repeat());
        themes.extend(background_size());
        themes.extend(background_image());
        themes.extend(gradient_color_stops());
        // borders
        themes.extend(border_radius());
        themes.extend(border_style());
        themes.extend(border_width());
        themes.extend(divide_style());
        themes.extend(divide_width());
        themes.extend(outline_offset());
        themes.extend(outline_style());
        themes.extend(outline_width());
        themes.extend(ring_offset_width());
        themes.extend(ring_style());
        themes.extend(ring_shadow_styles());
        // effects
        themes.extend(background_blend_mode());
        themes.extend(box_shadow());
        themes.extend(mix_blend_mode());
        themes.extend(opacity());
        // filters
        themes.extend(backdrop_blur());
        themes.extend(backdrop_brightness());
        themes.extend(backdrop_contrast());
        themes.extend(backdrop_grayscale());
        themes.extend(backdrop_hue_rotate());
        themes.extend(backdrop_invert());
        themes.extend(backdrop_opacity());
        themes.extend(backdrop_saturate());
        themes.extend(backdrop_sepia());
        themes.extend(blur());
        themes.extend(brightness());
        themes.extend(contrast());
        themes.extend(drop_shadow());
        themes.extend(grayscale());
        themes.extend(hue_rotate());
        themes.extend(invert());
        themes.extend(saturate());
        themes.extend(sepia());
        // tables
        themes.extend(border_collapse());
        themes.extend(border_spacing());
        themes.extend(table_layout());
        // transitions & animation
        themes.extend(animation());
        themes.extend(transition_delay());
        themes.extend(transition_duration());
        themes.extend(transition_property());
        themes.extend(transition_timing_function());
        // transforms
        themes.extend(rotate());
        themes.extend(scale());
        themes.extend(skew());
        themes.extend(transform_origin());
        themes.extend(translate());
        // interactivity
        themes.extend(appearance());
        themes.extend(cursor());
        themes.extend(pointer_events());
        themes.extend(resize());
        themes.extend(scroll_behavior());
        themes.extend(scroll_margin());
        themes.extend(scroll_padding());
        themes.extend(scroll_snap_align());
        themes.extend(scroll_snap_stop());
        themes.extend(scroll_snap_type());
        themes.extend(touch_action());
        themes.extend(user_select());
        themes.extend(will_change());
        // svg
        themes.extend(fill_colors_styles());
        themes.extend(stroke_colors_styles());
        themes.extend(stroke_width());

        themes
    }
}