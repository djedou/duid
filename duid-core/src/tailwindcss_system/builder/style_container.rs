use std::collections::HashSet;
use super::{TailwindStyle, BuildStyle};
use crate::tailwindcss_system::themes::{TailwindTheme, ThemeMode};

#[derive(Debug, Clone)]
pub(crate) struct StyleContainer {
    pub(crate) selectors: Vec<TailwindStyle>,
    pub(crate) themes: TailwindTheme,
}

impl StyleContainer {
    pub(crate) fn new() -> Self {
        StyleContainer {
            selectors: vec![TailwindStyle::new()],
            themes: TailwindTheme::new(ThemeMode::Light)
        }
    }

    pub(crate) fn build(&mut self, selectors: &HashSet<String>) -> Vec<(String, String)> {
        let mut styles = Vec::with_capacity(0);
        let latest = self.get_latest_selectors(&selectors);
        
        let latest_vec: Vec<_> = latest.iter().collect();
        let mut index = 0;
        for chunk in latest_vec.chunks(30) {
            let mut chunk_styles = Vec::with_capacity(0);
            chunk.iter().for_each(|&s| {
                let result = BuildStyle::build(&s, &self.themes.themes);
                if result.len() != 0 {
                    chunk_styles.push(result);
                }
            });
            
            let mut rebuild_set = HashSet::with_capacity(0);
            chunk.iter().for_each(|&s| {
                let _ = rebuild_set.insert(s.to_owned());
            });

            let builded_styles = chunk_styles.join(" ");
            self.selectors.push(TailwindStyle {
                is_full: chunk.len() > 27,
                selectors: rebuild_set,
                //style: builded_styles.clone(),
            });

            styles.push((format!("duid-style-{}", index), builded_styles));
            index = index + 1;
        }

        styles
    }

    pub(crate) fn render(&mut self, selectors: &HashSet<String>) -> Vec<(String, String)> {
        let mut styles = Vec::with_capacity(0);
        let latest = self.get_latest_selectors(&selectors);

        let latest_vec: Vec<_> = latest.iter().collect();
        let mut index = 0;
        for chunk in latest_vec.chunks(30) {
            let mut chunk_styles = Vec::with_capacity(0);

            chunk.iter().for_each(|&s| {
                let result = BuildStyle::build(&s, &self.themes.themes);
                if result.len() != 0 {
                    chunk_styles.push(result);
                }
            });
            
            let mut rebuild_set = HashSet::with_capacity(0);

            chunk.iter().for_each(|&s| {
                let _ = rebuild_set.insert(s.to_owned());
            });

            let builded_styles = chunk_styles.join(" ");
            if let Some(last) = self.selectors.last_mut() {
                if last.is_full {
                    self.selectors.push(TailwindStyle {
                        is_full: chunk.len() > 27,
                        selectors: rebuild_set,
                        //style: builded_styles.clone(),
                    });
                }
                else {
                    last.is_full = (chunk.len() + last.selectors.len()) > 27;
                    last.selectors.extend(rebuild_set);
                    //last.style.push_str(&builded_styles.clone());
                }
            }

            styles.push((format!("duid-style-{}", index), builded_styles));
            index = index + 1;
        }
        styles
    }

    fn get_latest_selectors(&self, selectors: &HashSet<String>) -> HashSet<String> {
        let mut latest = HashSet::with_capacity(0);

        for selector in selectors.iter()  {
            self.selectors.iter().for_each(|tailwind| {
                if !tailwind.contains(&selector) {
                    let _ = latest.insert(selector.to_owned());
                }
            });
        }
        latest
    }
}