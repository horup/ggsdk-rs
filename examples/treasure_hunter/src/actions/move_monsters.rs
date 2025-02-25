use crate::{Action, ThingVariant};

use super::LostAction;

pub struct MoveMonstersAction {}

impl Action for MoveMonstersAction {
    fn exec(self:Box<Self>, ctx: &mut crate::ActionContext) {
        let Some(player_index) = ctx.state.find_player() else {
            return;
        };
        for monster_index in ctx.state.find_monsters().drain(..) {
            let act;
            let Some(thing) = &mut ctx.state.grid.get_mut(monster_index).unwrap().thing else {
                continue;
            };
            let ThingVariant::Monster { think } = &mut thing.variant else {
                continue;
            };
            act = *think % 2 == 0;
            *think += 1;
            if !act {
                continue;
            }

            let mut path = ctx.state.path_find(monster_index, player_index);
            let Some(move_to) = path.pop_front() else {
                continue;
            };

            let perform_move;
            match ctx.state.grid.get(move_to) {
                Some(to_cell) => match &to_cell.thing {
                    Some(to_thing) => match to_thing.variant {
                        crate::ThingVariant::Player {} => {
                            perform_move = true;
                            ctx.push_action(LostAction {});
                        }
                        _ => perform_move = false,
                    },
                    None => perform_move = true,
                },
                None => perform_move = false,
            }
            if perform_move {
                let monster = ctx
                    .state
                    .grid
                    .get_mut(monster_index)
                    .unwrap()
                    .thing
                    .take()
                    .unwrap();

                ctx.state.grid.get_mut(move_to).unwrap().thing = Some(monster);
            }
        }
    }
}
