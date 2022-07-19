use std::fmt;
use super::Model;
use crate::event_manager::Message;



#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    key: i64,
    model: Model,
    update: fn(Model, Message) -> Model,
    subscriptions: Vec<Subscribe>,
}


#[derive(Clone)]
pub struct Subscribe {
    key: i64,
    subscribe: fn(&Model),
}

impl PartialEq for Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl fmt::Debug for Subscribe
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Subscribe: {:?}", self)
    }
}


impl Store {
    pub fn new(model: Model, update: fn(Model, Message) -> Model) -> Store {
        Store {
            key: 0,
            model: model,
            update: update,
            subscriptions: vec![],
        }
    }

    pub fn dispatch(&mut self, msg: Message) {
        let new = (self.update)(self.model.clone(), msg);
        if new != self.model {
            self.model = new;
            self.run();
        }
    }

    pub fn run(&self) {
        for sub in &self.subscriptions {
            (sub.subscribe)(&self.model)
        }
    }

    pub fn subscribe(&mut self, sub: fn(&Model)) -> i64 {
        self.key += 1;
        self.subscriptions.push(Subscribe {
            key: self.key,
            subscribe: sub,
        });
        return self.key;
    }

    pub fn unsubscribe(&mut self, sub: i64) {
        let index = self.subscriptions.iter().position(|x| x.key == sub);
        if let Some(i) = index {
            self.subscriptions.remove(i);
        }
    }
}