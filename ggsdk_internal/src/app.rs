use eframe::glow;
use crate::{GAssets, GGContext};

pub struct InitContext<'a> {
    pub assets: &'a mut GAssets,
    pub gl:&'a glow::Context,
}

pub trait GGApp {
    fn init(&mut self, g: &mut InitContext);
    fn update(&mut self, g: &mut GGContext);
}