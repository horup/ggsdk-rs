use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::{GAssets, GGApp, GGRunOptions, InitContext};
use eframe::{
    egui::{self, Align2, Color32, FontId, LayerId},
    egui_glow, glow,
};
use kira::AudioManager;
use web_time::Instant;

#[derive(Clone, Copy)]
pub enum GGEngineState {
    Preinit,
    Init,
    Postinit,
}

pub struct GGEngine {
    pub(crate) assets: ArcSendMutex<GAssets>,
    pub(crate) rhai_engine: rhai::Engine,
    pub(crate) rhai_ast: rhai::AST,
    pub(crate) audio_manager: AudioManager,
    pub(crate) iterations: u64,
    pub(crate) app: ArcSendMutex<dyn GGApp>,
    pub(crate) last_update: Instant,
    pub(crate) state: GGEngineState,
}

pub struct ArcSendMutex<T: ?Sized>(Arc<Mutex<T>>);
impl<T> ArcSendMutex<T> {
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }
}
impl<T: ?Sized> Clone for ArcSendMutex<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl<T: ?Sized> Send for ArcSendMutex<T> {}
unsafe impl<T: ?Sized> Sync for ArcSendMutex<T> {}
impl<T: ?Sized> Deref for ArcSendMutex<T> {
    type Target = Arc<Mutex<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: ?Sized> DerefMut for ArcSendMutex<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GGEngine {
    fn new<T: GGApp + 'static>(app: T) -> Self {
        let rhai_engine = rhai::Engine::new();
        let mut engine = Self {
            assets: ArcSendMutex::new(GAssets::default()),
            last_update: Instant::now(),
            app: ArcSendMutex(Arc::new(Mutex::new(app))),
            iterations: 0,
            rhai_engine,
            rhai_ast: Default::default(),
            audio_manager: AudioManager::new(Default::default())
                .expect("failed to initialize audio manager"),
            state: GGEngineState::Preinit,
        };

        engine.rhai_register_functions();

        engine
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_web() -> bool {
        false
    }

    #[cfg(target_arch = "wasm32")]
    pub fn is_web() -> bool {
        true
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run<T: GGApp + 'static>(app: T, options: GGRunOptions) {
        tracing_subscriber::fmt::init();
        let engine = Self::new(app);
        let size = options.window_initial_size.unwrap_or((640.0, 480.0));
        let eframe_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([size.0, size.1]),
            window_builder: Some(Box::new(move |window| {
                let mut window = window;
                if let Some(initial_pos) = options.window_initial_pos {
                    window = window.with_position(initial_pos);
                }
                if let Some(initial_active) = options.window_initial_active {
                    window = window.with_active(initial_active);
                }
                window
            })),
            ..Default::default()
        };
        eframe::run_native(
            &options.window_title,
            eframe_options,
            Box::new(|_| Ok(Box::new(engine))),
        )
        .unwrap();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run<T: GGApp + 'static>(game: T, options: GGRunOptions) {
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
            .without_time()
            .with_writer(tracing_web::MakeWebConsoleWriter::new())
            .init();

        tracing::debug!("hello world");

        use eframe::{App, wasm_bindgen::JsCast as _};
        wasm_bindgen_futures::spawn_local(async move {
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

            let canvas = document
                .get_element_by_id("main")
                .expect("Failed to find main")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("the_canvas_id was not a HtmlCanvasElement");

            if let Some(size) = options.window_initial_size {
                canvas.set_width(size.0 as u32);
                canvas.set_height(size.1 as u32);
            }

            let engine = Self::new(game);

            let web_options = eframe::WebOptions::default();
            let start_result = eframe::WebRunner::new()
                .start(canvas, web_options, Box::new(|__| Ok(Box::new(engine))))
                .await;
        });
    }

    pub fn update(&mut self, egui_ctx: &egui::Context, gl: &glow::Context) {
        let now = web_time::Instant::now();
        let dt = now - self.last_update;
        self.last_update = now;
        let dt = dt.as_secs_f32();

        self.assets.lock().unwrap().poll(crate::PollContext { egui_ctx: &egui_ctx });

        match self.state {
            GGEngineState::Preinit => {
                let painter =
                    egui_ctx.layer_painter(LayerId::new(egui::Order::Foreground, "preinit".into()));
                let clip = painter.clip_rect();
                let font = FontId::monospace(16.0);
                painter.text(
                    (clip.width() / 2.0, clip.height() / 2.0).into(),
                    Align2::CENTER_CENTER,
                    "FOCUS TO CONTINUE...",
                    font,
                    Color32::WHITE,
                );
                egui_ctx.input(|x| {
                    let any_input_events = x
                        .events
                        .iter()
                        .filter(|x| match x {
                            egui::Event::Key { .. } => true,
                            egui::Event::PointerButton { .. } => true,
                            egui::Event::Touch { .. } => true,
                            _ => false,
                        })
                        .count()
                        > 0;
                    if any_input_events {
                        // recreate audiomanager to ensure it works on the web
                        self.audio_manager = AudioManager::new(Default::default())
                            .expect("failed to initialize audio manager");
                        self.state = GGEngineState::Init;
                    }
                });
            }
            GGEngineState::Init => {
                self.app.lock().unwrap().init(InitContext {
                    assets: &mut self.assets.lock().unwrap(),
                    gl,
                });
                self.state = GGEngineState::Postinit;
            }
            GGEngineState::Postinit => {
                self.assets.lock().unwrap().poll(crate::PollContext { egui_ctx: &egui_ctx });

                self.app.lock().unwrap().update(crate::UpdateContext {
                    egui_ctx,
                    rhai_engine: &mut self.rhai_engine,
                    rhai_ast: &self.rhai_ast,
                    audio_manager: &mut self.audio_manager,
                    dt,
                    assets: &mut self.assets.lock().unwrap(),
                });

                let screen_rect = egui_ctx.screen_rect();
                let app = self.app.clone();
                let assets = self.assets.clone();
                let callback = egui::PaintCallback {
                    rect: screen_rect,
                    callback: std::sync::Arc::new(egui_glow::CallbackFn::new(
                        move |_info, painter| {
                            app.lock().unwrap().paint_glow(crate::PaintGlowContext {
                                dt,
                                assets: &mut assets.lock().unwrap(),
                                painter: painter,
                            });
                        },
                    )),
                };
                egui_ctx.layer_painter(LayerId::background()).add(callback);
            }
        }

        self.iterations += 1;
        egui_ctx.request_repaint();
    }

    pub fn load_script(&self, _path: &str) {}
    pub fn load_atlas(&self, _path: &str, _name: &str) {}
}

impl eframe::App for GGEngine {
    fn update(&mut self, ctx: &eframe::egui::Context, f: &mut eframe::Frame) {
        let gl = f.gl().expect("unable to get gl");
        self.update(ctx, gl);
    }
}
