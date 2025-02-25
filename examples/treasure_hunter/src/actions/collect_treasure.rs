use crate::{ActionContext, Action};

pub struct CollectTreasureAction {
}

impl Action for CollectTreasureAction {
    fn exec(self:Box<Self>, ctx:&mut ActionContext)  {
        let mut treasure_left = 0;
        for chunk in &mut ctx.state.grid {
            for (_, tile) in chunk {
                if let Some(thing) = &mut tile.thing {
                    if let crate::ThingVariant::Treasure {  } = thing.variant { treasure_left += 1 };
                }
            }
        }
        if treasure_left == 0 {
            for chunk in &mut ctx.state.grid {
                for (_, tile) in chunk {
                    if let Some(thing) = &mut tile.thing {
                        if let crate::ThingVariant::Grate {  } = thing.variant { tile.thing = None };
                    }
                }
            }
        }

        ctx.state.play_sound.push("pickup".to_string());
    }
}