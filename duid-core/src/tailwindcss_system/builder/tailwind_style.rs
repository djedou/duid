use std::collections::HashSet;

#[derive(Debug, Clone)]
pub(crate) struct TailwindStyle {
    pub(crate) is_full: bool,
    pub(crate) selectors: HashSet<String>,
    //pub(crate) style: String,
}

impl TailwindStyle {
    pub(crate) fn new() -> Self {
        TailwindStyle {
            is_full: false,
            selectors: HashSet::with_capacity(0),
            //style: String::with_capacity(0)
        }
    }

    pub(crate) fn contains(&self, selector: &str) -> bool {
        self.selectors.contains(selector)
    }
}