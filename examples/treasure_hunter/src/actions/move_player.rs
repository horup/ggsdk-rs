use crate::{ActionContext, Action};

use super::{CollectTreasureAction, FinishAction, LostAction, MoveMonstersAction};

#[derive(Clone)]
pub struct MovePlayerAction {
    pub dir: (i32, i32),
}

impl Action for MovePlayerAction {
    fn exec(self:Box<Self>, ctx:&mut ActionContext) {
        let Some(player_index) = ctx.state.find_player() else {
            return;
        };
        let to = (player_index.0 + self.dir.0, player_index.1 + self.dir.1);
        if let Some(to_cell) = ctx.state.grid.get(to) {
            let perform_move;
            if !to_cell.walls.is_empty() {
                perform_move = false;
            } else {
                match &to_cell.thing {
                    Some(to_thing) => match &to_thing.variant {
                        crate::ThingVariant::Player {} => {
                            perform_move = false;
                        }
                        crate::ThingVariant::Treasure {} => {
                            ctx.state.treasure_collected += 1;
                            ctx.push_action(CollectTreasureAction {});
                            perform_move = true;
                        }
                        crate::ThingVariant::Door {} => {
                            perform_move = true;
                            ctx.state.play_sound.push("open".to_string());
                        }
                        crate::ThingVariant::Grate {} => {
                            perform_move = false;
                        }
                        crate::ThingVariant::Exit {  } => {
                            perform_move = false;
                            ctx.push_action(FinishAction {});
                        },
                        crate::ThingVariant::Pit {  } => {
                            perform_move = false;
                            ctx.state.grid.get_mut(player_index).unwrap().thing.take();
                            ctx.push_action(LostAction {
                                
                            });
                        },
                        crate::ThingVariant::Monster { .. } => {
                            perform_move = false;
                            ctx.state.grid.get_mut(player_index).unwrap().thing.take();
                            ctx.push_action(LostAction {
                                
                            });
                        },
                    },
                    None => {
                        perform_move = true;
                    }
                }
            }

            if perform_move {
                let player = ctx.state.grid.get_mut(player_index).unwrap().thing.take();
                ctx.state.grid.get_mut(to).unwrap().thing = player;
                ctx.push_action(MoveMonstersAction {});
            }
        }
    }
}
