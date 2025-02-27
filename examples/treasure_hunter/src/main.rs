mod action;
pub use action::*;
mod state;
pub use state::*;
mod app;
pub use app::*;
pub mod actions;
use ggsdk::{GEngine, GRunOptions};

fn main() {
    let size = 16.0;
    let cell_size = 8.0 * 4.0;
    GEngine::run(TreasureHunter::default(), GRunOptions {
        window_title: "Treasure Hunter".to_string(),
        window_initial_size: Some((size * cell_size, size * cell_size)),
        ..Default::default()
    });
}