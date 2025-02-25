use crate::Action;

pub struct ShowMessageAction {
    pub length_sec:f32,
    pub msg:String
}

impl Action for ShowMessageAction {
    fn exec(mut self:Box<Self>, ctx:&mut crate::ActionContext) {
        ctx.state.msg = self.msg.clone();
        self.length_sec -= ctx.dt;
        if self.length_sec > 0.0 {
            ctx.push_action(ShowMessageAction {
                length_sec: self.length_sec.clone(),
                msg: self.msg.clone(),
            });
        } else {
            ctx.state.msg = Default::default();
        }
    }
}