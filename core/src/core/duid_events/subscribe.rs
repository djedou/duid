
pub struct Sub<MSG> {
    pub messages: Vec<MSG>
}

impl<MSG> Sub<MSG> {
    pub fn new(
        msg: MSG
    ) -> Self {

        Self {
            messages: vec![msg]
        }
    }

    pub fn batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {

        Self {
            messages: messages.into_iter().map(|msg| msg).collect()
        }
    }

    pub fn none() -> Self {
        Self {
            messages: Vec::with_capacity(0),
        }
    }

    pub fn map_sub_msg<MSG2>(self) -> Sub<MSG2>
    where
        MSG: 'static,
        MSG2: From<MSG>,
    {
        let Sub {
            messages
        } = self;

        Sub {
            messages: messages.into_iter().map(|msg| MSG2::from(msg)).collect()
        }
    }

    pub fn append_sub(
        mut self,
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {
        self.messages.extend(messages);
        self
    }

    pub fn merge_all(all_subs: Vec<Self>) -> Self {
        let mut messages = vec![];
        for sub in all_subs {
            messages.extend(sub.messages);
        }
        Sub::batch(messages)
    }
    
    pub fn extend(
        mut self,
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {
        self.messages.extend(messages);
        self
    }
}
