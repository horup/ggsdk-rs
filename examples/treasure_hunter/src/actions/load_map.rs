use endlessgrid::Grid;
use ggsdk::{endlessgrid, tiled};
use crate::{Action, Cell, Thing, ThingVariant};

use super::FadeAction;

pub struct LoadMapAction {
    pub map_name: String,
}

impl Action for LoadMapAction {
    fn exec(self:Box<Self>, ctx: &mut crate::ActionContext) {
        let Some(map) = ctx.assets.get::<tiled::Map>(&self.map_name) else {
            return;
        };

        let mut grid: Grid<Cell> = Grid::default();
        let map = &map.data;
        let next_level: String = map
            .properties
            .get("next_level")
            .and_then(|x| match x {
                tiled::PropertyValue::StringValue(v) => Some(v.clone()),
                _ => None,
            })
            .unwrap_or_default();
        let intro: String = map
            .properties
            .get("intro")
            .and_then(|x| match x {
                tiled::PropertyValue::StringValue(v) => Some(v.clone()),
                _ => None,
            })
            .unwrap_or_default();
        
        ctx.state.next_level = next_level;
        for layer in map.layers() {
            let class = layer.user_type.clone().unwrap_or_default();
            let Some(layer) = layer.as_tile_layer() else {
                continue;
            };
            let w = layer.width().unwrap() as i32;
            let h = layer.height().unwrap() as i32;
            for y in 0..h {
                for x in 0..w {
                    let Some(tile) = layer.get_tile(x, y) else {
                        continue;
                    };

                    if grid.get((x, y)).is_none() {
                        grid.insert((x, y), Default::default());
                    }

                    let cell = grid.get_mut((x, y)).unwrap();

                    match class.as_str() {
                        "floor" => {
                            cell.floor = tile.id() as u16;
                        }
                        "walls" => {
                            cell.walls.push(tile.id() as u16);
                        }
                        "things" => {
                            let atlas_index = tile.id() as u16;
                            let tile = tile.get_tile().unwrap();
                            let class = tile.user_type.clone().unwrap_or_default();
                            let variant = match class.as_str() {
                                "player" => ThingVariant::Player {},
                                "treasure" => ThingVariant::Treasure {},
                                "door" => ThingVariant::Door {},
                                "grate" => ThingVariant::Grate {},
                                "exit" => ThingVariant::Exit {},
                                "pit" => ThingVariant::Pit {},
                                "monster" => ThingVariant::Monster { think: 0 },
                                _ => panic!("unknown tile class: {class}"),
                            };
                            let thing = Thing {
                                variant,
                                atlas_index,
                            };
                            cell.thing = Some(thing);
                        }
                        _ => panic!("unknown layer class: {class}"),
                    };
                }
            }
        }

        ctx.state.won = false;
        ctx.state.grid = grid;
        ctx.state.show_menu = false;
        ctx.state.show_intro = intro.len() > 0;
        ctx.state.intro = intro;
        ctx.state.current_level = self.map_name.clone();
        ctx.state.msg = String::default();
        ctx.push_action(FadeAction {
            dir: super::FadeDirection::In,
        });
        ggsdk::persist::save("current_level", &ctx.state.current_level);
    }
}
