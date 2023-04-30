use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;


#[derive(Debug, PartialEq, Clone)]
pub struct LinkModel<D> {
    pub(crate) route: String,
    pub(crate) link_data: Rc<RefCell<Option<D>>>,
    pub(crate) selectors: HashSet<String>,
    pub(crate) classes: HashSet<String>,
}

impl<D> LinkModel<D> {
    pub fn new() -> LinkModel<D> {
        LinkModel {
            route: String::with_capacity(0),
            link_data: Rc::new(RefCell::new(None)),
            selectors: HashSet::with_capacity(0),
            classes: HashSet::with_capacity(0)
        }
    }

    pub fn add_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.remove(c.as_ref());
        });
    }

    pub fn remove_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.remove(c.as_ref());
        });
    }

    pub fn set_route(&mut self, route: impl ToString) {
        self.route = route.to_string();
    }

    pub fn get_route(&self) -> String {
        self.route.clone()
    }

    pub fn set_link_data(&mut self, data: D) {
        *self.link_data.borrow_mut() = Some(data);
    }

    pub fn get_link_data(&self) -> Rc<RefCell<Option<D>>> {
        Rc::clone(&self.link_data.clone())
    }
} 