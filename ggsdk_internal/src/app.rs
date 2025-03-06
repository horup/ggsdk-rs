use eframe::{egui_glow, glow};
use crate::{GAssets, GGContext};

pub struct InitContext<'a> {
    pub assets: &'a mut GAssets,
    pub gl:&'a glow::Context,
}

pub struct PaintGlow<'a> {
    pub dt:f32,
    pub assets: &'a mut GAssets,
    pub painter:&'a egui_glow::Painter
}

pub trait GGApp {
    fn init(&mut self, g: &mut InitContext);
    fn update(&mut self, g: &mut GGContext);
    fn paint_glow(&mut self, g:PaintGlow) {
        let _ = g;
    }
}