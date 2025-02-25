use crate::Action;

use super::{FadeAction, LoadMapAction};

pub struct FinishAction {
}

impl Action for FinishAction {
    fn exec(self:Box<Self>, ctx:&mut crate::ActionContext) {
        if ctx.state.next_level.is_empty() {
            ctx.state.won = true;
            ctx.state.msg = "You Won!".to_string();
            ctx.state.current_level = String::default();
            ggsdk::persist::save("current_level", &ctx.state.current_level);
            ctx.push_action(FadeAction {
                dir:super::FadeDirection::Out
            });
        } else {
            ctx.push_action(FadeAction {
                dir:super::FadeDirection::In
            });
            ctx.push_action(LoadMapAction {
                map_name:ctx.state.next_level.clone()
            });
            ctx.push_action(FadeAction {
                dir:super::FadeDirection::Out
            });
        }
    }
}