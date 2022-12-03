


#[derive(Debug, Clone)]
pub(crate) struct Normal<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) messages: Vec<MSG>,
}


impl<MSG> Normal<MSG> 
where
    MSG: std::fmt::Debug + Clone + 'static
{
    pub(crate) fn new() -> Self {
        Normal {
            messages: Vec::with_capacity(0)
        }
    }
}

