use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Debug, Clone)]
pub struct Model {
    pub data: Rc<RefCell<dyn Any>>
}


impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        ::core::ptr::eq(&*self.data, &*other.data)
    }
}


impl Model {
    pub fn new(model: Rc<RefCell<dyn Any>>) -> Self {
        Model {
            data: model
        }
    }
}

