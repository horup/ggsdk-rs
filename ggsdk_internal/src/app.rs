use eframe::{egui, egui_glow, glow};
use crate::GAssets;

pub struct InitContext<'a> {
    pub assets: &'a mut GAssets,
    pub gl:&'a glow::Context,
}

pub struct PaintGlowContext<'a> {
    pub dt:f32,
    pub assets: &'a mut GAssets,
    pub painter:&'a egui_glow::Painter,
}

pub struct UpdateContext<'a> {
    pub assets: &'a mut GAssets,
    pub egui_ctx: &'a egui::Context,
    pub rhai_engine: &'a mut rhai::Engine,
    pub rhai_ast: &'a rhai::AST,
    pub audio_manager:&'a mut kira::AudioManager,
    pub dt:f32,
}

pub trait GGApp {
    /// happens once at the start of the application
    fn init(&mut self, g: InitContext);

    /// happens every frame, after paint_glow
    fn update(&mut self, g: UpdateContext);

    /// happens every frame before paint_glow
    fn update_glow(&mut self, g: UpdateContext) {
        let _ = g;
    }

    /// happens every frame to paint via glow, between update_glow and update
    fn paint_glow(&mut self, g:PaintGlowContext) {
        let _ = g;
    }
}