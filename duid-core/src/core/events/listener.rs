use std::any::TypeId;
use std::{fmt, rc::Rc};


pub struct Listener<IN, OUT> {
    func: Rc<dyn Fn(IN) -> OUT>,
    func_type_id: TypeId,
    event_type_id: TypeId,
    msg_type_id: TypeId,
}

impl<IN, OUT, F> From<F> for Listener<IN, OUT>
where
    F: Fn(IN) -> OUT + 'static,
    IN: 'static,
    OUT: 'static
{
    fn from(func: F) -> Self {
        Self {
            func: Rc::new(func),
            func_type_id: TypeId::of::<F>(),
            event_type_id: TypeId::of::<IN>(),
            msg_type_id: TypeId::of::<OUT>(),
        }
    }
}


impl<IN, OUT> fmt::Debug for Listener<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "in: {:?}, out: {:?}, func: {:?}",
            self.event_type_id, self.msg_type_id, self.func_type_id
        )
    }
}

impl<IN, OUT> Listener<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
{
    pub fn emit(&self, input: IN) -> OUT {
        (self.func)(input)
    }
    
    pub fn map_callback<MSG2>(
        self,
        cb: Listener<OUT, MSG2>,
    ) -> Listener<IN, MSG2>
    where
        MSG2: 'static,
    {
        let func_wrap = move |input| {
            let out = self.emit(input);
            cb.emit(out)
        };
        Listener::from(func_wrap)
    }
}


impl<IN, OUT> Clone for Listener<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            func: Rc::clone(&self.func),
            func_type_id: self.func_type_id,
            event_type_id: self.event_type_id,
            msg_type_id: self.msg_type_id,
        }
    }
}


impl<IN, OUT> PartialEq for Listener<IN, OUT> {
    fn eq(&self, other: &Self) -> bool {
        self.event_type_id == other.event_type_id
            && self.msg_type_id == other.msg_type_id
            && self.func_type_id == other.func_type_id
    }
}