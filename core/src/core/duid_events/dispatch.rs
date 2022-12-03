
pub trait Dispatch<MSG> {
    fn dispatch(&self, msg: MSG);

    fn dispatch_multiple(&self, msgs: Vec<MSG>);
    
    fn dispatch_with_delay(&self, msg: MSG, timeout: i32) -> Option<i32>;
}