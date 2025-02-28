use crate::GGContext;

pub trait GGApp {
    fn init(&mut self, g: &mut GGContext);
    fn update(&mut self, g: &mut GGContext);
}