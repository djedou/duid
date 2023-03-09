use std::collections::HashMap;
use css_colors::{HSL, hsl, Ratio, Color};
use crate::tailwindcss_system::themes::ThemeMode;

pub(crate) struct BuildStyle;

impl BuildStyle {
    pub(crate) fn build(data: &str, themes: &HashMap<String, String>, mode: &ThemeMode) -> (String, String) {
        // FORMAT: media_query:::css_selectors:::tailwind_css_properties
        let data_splited: Vec<_> = data.split(":::").collect();
        
        match data_splited.len() {
            2 => {
                let styles: Vec<_> = data_splited[1].split(" ").map(|v| {
                    match v.contains("[") {
                        true => {
                            let func_val: Vec<_> = v.split("[").collect();
                            // replace & by " ", so in [] & is equal to " ": i.e: calc(100px&-&2px) <=> calc(100px - 2px)
                            BuildStyle::from_function(&func_val[0], &func_val[1].replacen("]", "", 1).replace("&", " "), &themes, &mode)
                        },
                        false => BuildStyle::from_value(v, &themes, &mode)
                    }
                })
                .filter(|s| s.len() != 0)
                .collect();

                ( data_splited[0].to_owned(), format!("{} {{{}}}", data_splited[0], styles.join("")))
            },
            3 => {
                let styles: Vec<_> = data_splited[2].split(" ").map(|v| {
                    match v.contains("[") {
                        true => {
                            let func_val: Vec<_> = v.split("[").collect();
                            BuildStyle::from_function(&func_val[0], &func_val[1].replacen("]", "", 1).replace("&", " "), &themes, &mode)
                        },
                        false => BuildStyle::from_value(v, &themes, &mode)
                    }
                })
                .filter(|s| s.len() != 0)
                .collect();

                match BuildStyle::responsive(data_splited[0]) {
                    None => (data_splited[1].to_owned() , format!("{} {{{}}}", data_splited[1], styles.join(""))),
                    Some(val) => (format!("{}:::{}", data_splited[0], data_splited[1]), format!("{} {{{}}}", val, format!("{} {{{}}}", data_splited[1], styles.join(""))))
                }
            },
            _ => (String::with_capacity(0), String::with_capacity(0))
        }
    }

    fn from_value(name: &str, themes: &HashMap<String, String>, mode: &ThemeMode) -> String {
        match name.contains("color") {
            true => BuildStyle::get_color_property(&name, &themes, &mode),
            false => {
                match themes.get(name) {
                    Some(theme) => theme.to_owned(),
                    None => String::with_capacity(0)
                }
            }
        }
    }

    fn from_function(name: &str, value: &str, themes: &HashMap<String, String>, mode: &ThemeMode) -> String {
        
        match name.contains("color") { 
            true => BuildStyle::get_color_property(&name, &themes, &mode),
            false => {
                match themes.get(name) {
                    Some(theme) => {
                        let theme_splited: Vec<_> = theme.split(":").collect();
                        format!("{}: {};", theme_splited[0], value)
                    },
                    None => String::with_capacity(0)
                }
            }
        }
    }

    fn responsive(media: &str) -> Option<String> {
        match media {
            "sm" => Some(String::from("@media (min-width: 576px)")), // 576px and up
            "md" => Some(String::from("@media (min-width: 768px)")), // 768px and up
            "lg" => Some(String::from("@media (min-width: 992px)")), // 992px and up
            "xl" => Some(String::from("@media (min-width: 1200px)")), // 1200px and up
            "2xl" => Some(String::from("@media (min-width: 1400px)")), // 1400px and up
            _ => None,
        }
    }

    fn get_color_property(input: &str, themes: &HashMap<String, String>, mode: &ThemeMode) -> String {
        // color_name--h_name-s_value-l_value
        // color_name--h_vlaue-s_value-l_value
        // color_name--custom_name

        let input_splited: Vec<_> = input.split("--").collect();    
        match themes.get(input_splited[0]) {
            Some(property) => {
                format!("{}: {};", property, BuildStyle::get_color_value(&input_splited[1], &mode))
            },
            None => String::with_capacity(0)
        }
    }

    fn get_color_value(input: &str, mode: &ThemeMode) -> String {
        match input {
            "inherit" => "inherit".to_string(),
            "current" => "currentColor".to_string(),
            "transparent" => "transparent".to_string(),
            "black" => {
                match mode {
                    ThemeMode::Light => {
                    let color = hsl(0, 0, 0);
                    color.to_css()
                    },
                    ThemeMode::Dark => {
                        let color = hsl(0, 100, 100);
                        color.to_css()
                    }
                }
            },
            "white" => {
                match mode {
                    ThemeMode::Light => {
                    let color = hsl(0, 100, 100);
                    color.to_css()
                    },
                    ThemeMode::Dark => {
                        let color = hsl(0, 0, 0);
                        color.to_css()
                    }
                }
            },
            _ => {
                let input_splited: Vec<_> = input.split("-").collect();
                match input_splited.len() == 3 {
                    false => {
                        let color = hsl(0, 0, 0);
                        color.to_css()
                    },
                    true => {
                        match input_splited[0] {
                            "red" => {
                                if let (Ok(s), Ok(l)) = (input_splited[1].parse::<u8>(), input_splited[2].parse::<u8>()) {
                                    BuildStyle::get_color_by_mode(hsl(0, s, l), &mode)
                                }
                                else {
                                    BuildStyle::get_color_by_mode(hsl(0, 0, 0), &mode)
                                }
                            },
                            _ => {
                                if let (Ok(h), Ok(s), Ok(l)) = (input_splited[0].parse::<i32>(), input_splited[1].parse::<u8>(), input_splited[2].parse::<u8>()) {
                                    BuildStyle::get_color_by_mode(hsl(h, s, l), &mode)
                                }
                                else {
                                    BuildStyle::get_color_by_mode(hsl(0, 0, 0), &mode)
                                } 
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_color_by_mode(color: HSL, mode: &ThemeMode) -> String {
        match mode {
            ThemeMode::Light => {
                color.to_css()
            },
            ThemeMode::Dark => {
                let color = color.darken(Ratio::from_percentage(50));
                color.to_css()
            }
        }
    }
}
