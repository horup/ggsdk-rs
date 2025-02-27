use eframe::egui::{ColorImage, Context, TextureHandle, TextureOptions};

#[derive(Clone)]
pub struct GAtlas {
    pub texture:TextureHandle,
    pub cols:u8,
    pub rows:u8,
    pub name:String
}

impl GAtlas {
    pub fn new(ctx: &Context, name:impl Into<String>, image_data: &[u8], cols:u8, rows:u8) -> Self {
        let name = name.into();
        let img = image::load_from_memory(image_data).unwrap();
        let size = [img.width() as _, img.height() as _];
        let image_buffer = img.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let img = ColorImage::from_rgba_premultiplied(size, pixels.as_slice());
        let img = ctx.load_texture(name.clone(), img, TextureOptions::NEAREST);
        
        Self {
            name,
            texture:img,
            cols,
            rows
        }
    }
}
