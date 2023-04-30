use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq, Clone)]
pub enum LinkMsg<D, M> {
    OnClick(Rc<RefCell<Option<D>>>),
    ChildMsg(M)
}