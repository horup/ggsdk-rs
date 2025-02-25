use eframe::egui::{Color32, Painter, Rect};

use crate::GAtlas;

pub trait GPainter {
    fn atlas(&self, atlas: &GAtlas, index: u16, rect: Rect, color: Color32);
}

impl GPainter for Painter {
    fn atlas(&self, atlas: &GAtlas, index: u16, rect: Rect, color: Color32) {
        let sw: f32 = 1.0 / atlas.cols as f32;
        let sh = 1.0 / atlas.rows as f32;
        let col = index % atlas.cols as u16;
        let row = index / atlas.cols as u16;
        let sx = col as f32 / atlas.cols as f32;
        let sy = row as f32 / atlas.rows as f32;
        let src = Rect::from_min_max((sx, sy).into(), (sx + sw, sy + sh).into());
        self.image(atlas.texture.id(), rect, src, color);
    }
}
