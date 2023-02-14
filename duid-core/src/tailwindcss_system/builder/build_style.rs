use std::collections::HashMap;


pub(crate) struct BuildStyle;

impl BuildStyle {
    pub(crate) fn build(data: &str, themes: &HashMap<String, String>) -> String {
        // FORMAT: media_query:::css_selectors:::tailwind_css_properties
        let data_splited: Vec<_> = data.split(":::").collect();
        
        match data_splited.len() {
            2 => {
                let styles: Vec<_> = data_splited[1].split(" ").map(|v| {
                    match v.contains("[") {
                        true => {
                            let func_val: Vec<_> = v.split("[").collect();
                            // replace & by " ", so in [] & is equal to " ": i.e: calc(100px&-&2px) <=> calc(100px - 2px)
                            BuildStyle::from_function(&func_val[0], &func_val[1].replacen("]", "", 1).replace("&", " "), &themes)
                        },
                        false => BuildStyle::from_value(v, &themes)
                    }
                })
                .filter(|s| s.len() != 0)
                .collect();

                format!("{} {{{}}}", data_splited[0], styles.join(""))
            },
            3 => {
                let styles: Vec<_> = data_splited[2].split(" ").map(|v| {
                    match v.contains("[") {
                        true => {
                            let func_val: Vec<_> = v.split("[").collect();
                            BuildStyle::from_function(&func_val[0], &func_val[1].replacen("]", "", 1).replace("&", " "), &themes)
                        },
                        false => BuildStyle::from_value(v, &themes)
                    }
                })
                .filter(|s| s.len() != 0)
                .collect();

                match BuildStyle::responsive(data_splited[0]) {
                    None => format!("{} {{{}}}", data_splited[1], styles.join("")),
                    Some(val) => format!("{} {{{}}}", val, format!("{} {{{}}}", data_splited[1], styles.join("")))
                }
            },
            _ => String::with_capacity(0)
        }
    }

    fn from_value(name: &str, themes: &HashMap<String, String>) -> String {
        match themes.get(name) {
            Some(theme) => theme.to_owned(),
            None => String::with_capacity(0)
        }
    }

    fn from_function(name: &str, value: &str, themes: &HashMap<String, String>) -> String {
        match themes.get(name) {
            Some(theme) => {
                let theme_splited: Vec<_> = theme.split(":").collect();
                format!("{}: {};", theme_splited[0], value)
            },
            None => String::with_capacity(0)
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
}
