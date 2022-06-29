use crate::program::ports::Modifier;


pub struct Cmd<MSG> 
where MSG: 'static + std::fmt::Debug
{
    /// Messages that will be executed locally in the Component
    pub messages: Vec<MSG>,
    pub(crate) modifier: Modifier,
}

impl<MSG> Cmd<MSG> 
where MSG: 'static + std::fmt::Debug
{
    /// create a new Cmd with one message
    pub fn new(
        message: MSG
    ) -> Self {
        Self {
            messages: vec![message],
            modifier: Modifier::default(),
        }
    }

    /// Create and empty Cmd
    pub fn none() -> Self {
        Self {
            messages: vec![],
            modifier: Modifier::default(),
        }
    }

    /// create a new Cmd with more messages
    pub fn batch(
        messages: impl IntoIterator<Item = MSG>
    ) -> Self {
        Self {
            messages: messages.into_iter().collect(),
            modifier: Modifier::default(),
        }
    }

    /// Append this msgs to the cmd
    pub fn append(
        mut self,
        messages: impl IntoIterator<Item = MSG>,
    ) -> Self {
        self.messages.extend(messages);
        self
    }

    /// Modify the Effect such that it will not do an update on the view when it is executed
    pub fn no_render(mut self) -> Self {
        self.modifier.should_update_view = false;
        self
    }

    /// Merge all the internal objects of this Vec of Effects to produce only one.
    pub fn merge_all(all_effects: Vec<Self>) -> Self {
        let mut messages = vec![];
        for effect in all_effects {
            messages.extend(effect.messages);
        }
        Cmd::batch(messages)
    }

    /// Extern the cmd
    pub fn extend(
        mut self,
        messages: impl IntoIterator<Item = MSG>,
    ) -> Self {
        self.messages.extend(messages);
        self
    }
}