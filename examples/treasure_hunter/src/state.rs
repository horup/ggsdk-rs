use std::collections::VecDeque;

use endlessgrid::Grid;
use gsdk::endlessgrid;

use crate::Action;

#[derive(Clone)]
pub enum ThingVariant {
    Player {},
    Treasure {},
    Door {},
    Grate {},
    Exit {},
    Pit {},
    Monster {
        think:u32
    },
}

#[derive(Clone)]
pub struct Thing {
    pub variant: ThingVariant,
    pub atlas_index: u16,
}

#[derive(Default, Clone)]
pub struct Cell {
    pub thing: Option<Thing>,
    pub walls: Vec<u16>,
    pub floor: u16,
}

#[derive(Default)]
pub struct State {
    pub grid: Grid<Cell>,
    pub intro: String,
    pub show_intro: bool,
    pub next_level: String,
    pub treasure_collected: u32,
    pub play_sound: Vec<String>,
    pub fade: f32,
    pub show_menu: bool,
    pub actions: Vec<Box<dyn Action>>,
    pub current_level: String,
    pub won: bool,
    pub msg: String
}

impl State {
    pub fn find_player(&self) -> Option<(i32, i32)> {
        for chunk in &self.grid {
            for (index, cell) in chunk {
                if let Some(thing) = &cell.thing {
                    if let ThingVariant::Player {} = thing.variant {
                        return Some(index);
                    }
                }
            }
        }

        None
    }

    pub fn find_monsters(&self) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        for chunk in &self.grid {
            for (index, cell) in chunk {
                if let Some(thing) = &cell.thing {
                    if let ThingVariant::Monster { .. } = thing.variant {
                        res.push(index);
                    }
                }
            }
        }

        res
    }

    pub fn path_find(&self, start: (i32, i32), end: (i32, i32)) -> VecDeque<(i32, i32)> {
        let mut path = self
            .grid
            .astar(start, end, |visit| {
                if visit.index == start || visit.index == end {
                    return false;
                }
                if visit.cell.walls.len() == 0 {
                    return match &visit.cell.thing {
                        Some(thing) => match thing.variant {
                            ThingVariant::Player {  } => false,
                            _ => true,
                        },
                        None => false,
                    };
                }

                return true;
            })
            .unwrap_or_default();

        let mut res = VecDeque::with_capacity(path.len());
        if path.len() > 0 {
            for path in path.drain(1..) {
                res.push_back(path);
            }
        }

        res
    }

    pub fn input_allowed(&self) -> bool {
        if self.fade != 0.0 {
            return false;
        }

        if self.show_menu {
            return false;
        }

        if self.actions.len() != 0 {
            return false;
        }

        if self.won == false {}

        true
    }
}
