use crate::Action;

use super::{FadeAction, LoadMapAction, ShowMessageAction};

pub struct LostAction {

}

impl Action for LostAction {
    fn exec(self:Box<Self>, ctx:&mut crate::ActionContext) {
        ctx.push_action(LoadMapAction {
            map_name:ctx.state.current_level.clone()
        });
        ctx.push_action(FadeAction {
            dir:super::FadeDirection::Out
        });
        ctx.push_action(ShowMessageAction {
            length_sec:3.0,
            msg:"You Died!".to_string()
        });
        ctx.push_action(FadeAction {
            dir:super::FadeDirection::In
        });
    }
}