use eframe::{egui, egui_glow, glow};
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

pub struct Update<'a> {
    pub assets: &'a mut GAssets,
    pub egui_ctx: &'a egui::Context,
    pub rhai_engine: &'a mut rhai::Engine,
    pub rhai_ast: &'a rhai::AST,
    pub audio_manager:&'a mut kira::AudioManager,
    pub dt:f32,
}

pub trait GGApp {
    fn init(&mut self, g: InitContext);
    fn update(&mut self, g: Update);
    fn paint_glow(&mut self, g:PaintGlow) {
        let _ = g;
    }
}