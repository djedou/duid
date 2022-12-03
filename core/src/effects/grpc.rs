#[derive(Debug, Clone)]
pub(crate) struct Grpc<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) messages: Vec<MSG>,
}


impl<MSG> Grpc<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) fn new() -> Self {
        Grpc {
            messages: Vec::with_capacity(0)
        }
    }
}
