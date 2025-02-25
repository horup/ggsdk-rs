use ggsdk::GAssets;

use crate::State;

pub struct ActionContext<'a> { 
    pub state:&'a mut State,
    pub new_actions:Vec<Box<dyn Action>>,
    pub assets:&'a GAssets,
    pub dt:f32
}

impl ActionContext<'_> {
    pub fn push_action<T:Action + 'static>(&mut self, intent:T) {
        self.new_actions.push(Box::new(intent));
    }
}

pub trait Action {
    fn exec(self:Box<Self>, ctx:&mut ActionContext);
}
