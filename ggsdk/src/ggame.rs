use crate::GContext;

pub trait GGame {
    fn init(&mut self, g: &mut GContext);
    fn update(&mut self, g: &mut GContext);
}