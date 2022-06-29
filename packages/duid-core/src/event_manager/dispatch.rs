
pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG);
    
    fn dispatch_multiple(&self, msgs: Vec<MSG>);
}