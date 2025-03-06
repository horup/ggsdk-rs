use ggsdk::{
    egui::{self, Align2, Button, Color32, CornerRadius, FontId, LayerId, Margin, Rect, RichText}, kira, tiled, GGApp, GGAtlas, GGContext, GGPainter, Update
};
use kira::sound::static_sound::StaticSoundData;
use std::{cell::RefCell, rc::Rc};

use crate::{
    actions::{self, FadeAction, LoadMapAction},
    Action, ActionContext, State,
};

#[derive(Default)]
pub struct TreasureHunter {
    pub initialized: bool,
    pub font: FontId,
    pub font2: FontId,
    pub state: Rc<RefCell<State>>,
}

impl TreasureHunter {
    pub fn initialize(&mut self, g: &mut Update) {
        if !self.initialized && g.assets.pending() == 0 {
            self.initialized = true;
        }
    }

    pub fn process_game_input(&mut self, g: &mut Update) {
        let ctx = g.egui_ctx;
        ctx.input(|input| {
            if input.key_pressed(egui::Key::Escape) {
                let mut state = self.state.borrow_mut();
                let show_menu = state.show_menu;
                state.show_menu = !show_menu;
            }
        });
        if self.state.borrow().input_allowed() == false {
            return;
        }
        ctx.input(|input| {
            let mut dx = 0;
            let mut dy = 0;
            if input.key_pressed(egui::Key::Space) {
                self.push_action(actions::MoveMonstersAction {});
                return;
            }
            if input.key_pressed(egui::Key::A) {
                dx = -1;
                dy = 0;
            }
            if input.key_pressed(egui::Key::D) {
                dx = 1;
                dy = 0;
            }
            if input.key_pressed(egui::Key::W) {
                dy = -1;
                dx = 0;
            }
            if input.key_pressed(egui::Key::S) {
                dy = 1;
                dx = 0;
            }

            if input.key_pressed(egui::Key::Num1) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl01".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num2) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl02".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num3) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl03".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num4) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl04".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num5) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl05".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num6) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl06".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num7) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl07".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num8) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl08".to_string(),
                });
                return;
            }
            if input.key_pressed(egui::Key::Num9) {
                self.push_action(actions::LoadMapAction {
                    map_name: "lvl09".to_string(),
                });
                return;
            }

            if dx != 0 || dy != 0 {
                self.push_action(actions::MovePlayerAction { dir: (dx, dy) });
            }
        });
    }

    fn push_action<T: Action + 'static>(&mut self, intent: T) {
        self.state.borrow_mut().actions.push(Box::new(intent));
    }

    fn process_actions(&mut self, g: &mut Update) {
        let mut ctx = ActionContext {
            state: &mut self.state.borrow_mut(),
            new_actions: Vec::default(),
            assets: g.assets,
            dt: g.dt,
        };
        if let Some(action) = ctx.state.actions.pop() {
            action.exec(&mut ctx);
        }
        for new_actions in ctx.new_actions.drain(..) {
            ctx.state.actions.push(new_actions);
        }
    }

    fn play_sounds(&mut self, g: &mut Update) {
        for sound in self.state.borrow_mut().play_sound.drain(..) {
            let Some(sound) = g.assets.get::<StaticSoundData>(&sound) else {
                continue;
            };
            let _ = g.audio_manager.play(sound.data.clone());
        }
    }

    fn draw_game(&self, g: &mut Update) {
        let painter = g.egui_ctx.layer_painter(LayerId::background());
        let rect = painter.clip_rect();


        if g.assets.pending() > 0 {
            painter.rect_filled(rect, CornerRadius::ZERO, Color32::BLACK);
            let s = format!("{} / {}", g.assets.loaded(), g.assets.total());
            painter.text(
                (rect.width() / 2.0, rect.height() / 2.0 - self.font.size).into(),
                Align2::CENTER_CENTER,
                "Loading",
                self.font.clone(),
                Color32::WHITE,
            );
            painter.text(
                (rect.width() / 2.0, rect.height() / 2.0).into(),
                Align2::CENTER_CENTER,
                s,
                self.font.clone(),
                Color32::WHITE,
            );

            return;
        }

        // draw void
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::DARK_GRAY);

        let Some(atlas) = g.assets.get::<GGAtlas>("basic") else {
            return;
        };
        let atlas = &atlas.data;
        let state = self.state.borrow();

        fn calc_rect(x: i32, y: i32) -> Rect {
            let size = 32.0;
            Rect::from_min_size(
                (x as f32 * size, y as f32 * size).into(),
                (size, size).into(),
            )
        }


        // draw floor
        for chunk in &state.grid {
            for ((x, y), cell) in chunk {
                painter.atlas(atlas, cell.floor, calc_rect(x, y), Color32::WHITE);
            }
        }

        // draw things
        for chunk in &state.grid {
            for ((x, y), cell) in chunk {
                if let Some(thing) = &cell.thing {
                    painter.atlas(atlas, thing.atlas_index, calc_rect(x, y), Color32::WHITE);
                }
            }
        }

        // draw walls
        for chunk in &state.grid {
            for ((x, y), cell) in chunk {
                for wall in cell.walls.iter() {
                    painter.atlas(atlas, *wall, calc_rect(x, y), Color32::WHITE);
                }
            }
        }

        // draw intro text
        let intro = state.intro.clone();
        let margin = 16.0;
        let galley_width = painter.clip_rect().width() - margin * 2.0;
        let galley = painter.layout(intro, self.font2.clone(), Color32::WHITE, galley_width);
        painter.galley((margin, margin).into(), galley, Color32::WHITE);

        // draw fade
        painter.rect_filled(
            rect,
            CornerRadius::ZERO,
            Color32::from_black_alpha((255.0 * state.fade) as u8),
        );

        // draw msg
        if state.msg.len() > 0 {
            painter.rect_filled(
                Rect::from_center_size((rect.width() / 2.0, rect.height() / 2.0).into(), (rect.width(), 32.0).into()),
                CornerRadius::ZERO,
                Color32::from_black_alpha((255.0 * 0.5) as u8),
            );
            painter.text(
                (rect.width() / 2.0, rect.height() / 2.0).into(),
                Align2::CENTER_CENTER,
                state.msg.clone(),
                self.font.clone(),
                Color32::WHITE//Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * state.fade) as u8),
            );
        }
      
    }

    pub fn update_ui(&mut self, g: &mut Update) {
        let show_menu = self.state.borrow().show_menu;
        self.process_game_input(g);
        self.process_actions(g);
        self.draw_game(g);
        self.play_sounds(g);

        if g.assets.pending() > 0 {
            return;
        }
        if show_menu {
            let current_level = self.state.borrow().current_level.to_string();

            egui::CentralPanel::default()
                .frame(egui::Frame {
                    fill: Color32::from_rgba_unmultiplied(64, 64, 64, 128),
                    outer_margin: Margin::symmetric(64, 64),
                    inner_margin: Margin::symmetric(32, 32),
                    ..Default::default()
                })
                .show(g.egui_ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("Treasure Hunter")
                                .color(Color32::WHITE)
                                .size(32.0),
                        );
                        ui.add_space(32.0);
                        if ui
                            .add_enabled(
                                current_level.len() != 0,
                                Button::new(RichText::new("   Continue   ").size(16.0)),
                            )
                            .clicked()
                        {
                            self.push_action(FadeAction::fade_in());
                            self.push_action(LoadMapAction {
                                map_name: current_level,
                            });
                            self.push_action(FadeAction::fade_out());
                            self.state.borrow_mut().show_menu = false;
                        }
                        ui.add_space(16.0);
                        if ui
                            .button(RichText::new("   New Game   ").size(16.0))
                            .clicked()
                        {
                            self.push_action(FadeAction::fade_in());
                            self.push_action(LoadMapAction {
                                map_name: "lvl01".to_string(),
                            });
                            self.push_action(FadeAction::fade_out());
                            self.state.borrow_mut().show_menu = false;
                        }
                    });
                });
        }
    }
}

impl GGApp for TreasureHunter {
    fn init(&mut self, mut g: ggsdk::InitContext) {
        let font = FontId::monospace(32.0);
        self.font = font;
        let font = FontId::monospace(16.0);
        self.font2 = font;
        g.assets.load::<GGAtlas>("assets/basic_32x32.png", "basic");
        for i in 1..=9 {
            let i = i.to_string();
            g.assets.load::<tiled::Map>(&format!("assets/maps/lvl0{i}.tmx"), &format!("lvl0{i}"));
        }
        g.assets.load::<StaticSoundData>("assets/sfx/coin.mp3", "pickup");
        g.assets.load::<StaticSoundData>("assets/sfx/open.mp3", "open");
        let mut state = self.state.borrow_mut();
        state.fade = 1.0;
        state.show_menu = true;
        state.current_level = ggsdk::persist::load::<String>("current_level").unwrap_or_default();
    }

    fn update(&mut self, mut g: ggsdk::Update) {
        self.initialize(&mut g); 
        self.update_ui(&mut g);
    }
}
