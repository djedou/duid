use std::collections::{HashMap, HashSet};
use super::BuildStyle;
use crate::tailwindcss_system::themes::{TailwindTheme, ThemeMode};

#[derive(Debug, Clone)]
pub(crate) struct StyleContainer {
    pub(crate) selectors: HashSet<String>,
    pub(crate) themes: TailwindTheme,
}

impl StyleContainer {
    pub(crate) fn new() -> Self {
        StyleContainer {
            selectors: HashSet::new(),
            themes: TailwindTheme::new(ThemeMode::Light)
        }
    }

    pub(crate) fn build(&mut self, selectors: &HashMap<usize, HashSet<String>>) -> Vec<(String, String)> {
        let mut full_styles = Vec::with_capacity(0);
        let mut styles = Vec::with_capacity(0);
        
        for (key, values) in selectors.iter() {
            let mut loop_break = false;
            let mut partial_styles = Vec::with_capacity(0);
            let mut partial_selectors = HashSet::with_capacity(0);

            for (properties_selector, properties) in self.build_styles(values) {
                //let mut inner_loop_break = false;
                match self.selectors.contains(&properties_selector) {
                    true => {
                        loop_break = true;
                        break;
                    },
                    false => {
                        partial_styles.push(properties);
                        partial_selectors.insert(properties_selector);
                    }
                }
            }

            match loop_break {
                true => {
                    continue;
                },
                false => {
                    self.selectors.extend(partial_selectors);
                    full_styles.push((*key, partial_styles.join(" ")));
                }
            }
        }


        for chunk in full_styles.chunks(40) {
            let chunk_styles: Vec<_> = chunk.iter().map(|(_, v)| v.to_owned()).collect();
            let chunk_keys: Vec<_> = chunk.iter().map(|(k, _)| k.to_string()).collect();

            let builded_styles = chunk_styles.join(" ");
            let builded_keys = chunk_keys.join("");
            

            styles.push((format!("duid-style-{}", builded_keys), builded_styles));
        }
        
        styles
    }

    fn build_styles(&self, selectors: &HashSet<String>) -> Vec<(String, String)> {
        let mut result = vec![];
        let mut media_result = vec![];

        let selectors_vec: Vec<_> = selectors.iter().collect();
        for style in selectors_vec {
            let (style_selector, builded_style) = BuildStyle::build(&style, &self.themes.themes);

            match (builded_style.len() != 0, !builded_style.contains("@media")) {
                (true, true) => {
                    result.push((style_selector , builded_style));
                },
                (true, false) => {
                    media_result.push((style_selector, builded_style));
                },
                _ => {}
            }
        }
        
        result.extend_from_slice(&media_result);
        result
    }
}