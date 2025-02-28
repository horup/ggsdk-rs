use eframe::egui;
use rhai::CallFnOptions;
use tracing::warn;
use crate::GAssets;

pub struct GGContext<'a> {
    pub assets: &'a mut GAssets,
    pub egui_ctx: &'a egui::Context,
    pub rhai_engine: &'a mut rhai::Engine,
    pub rhai_ast: &'a rhai::AST,
    pub audio_manager:&'a mut kira::AudioManager,
    pub dt:f32,
}

impl GGContext<'_> {
    pub fn call_function<T : Clone + 'static>(&self, name: &str, args: impl rhai::FuncArgs) -> Result<T, String>  {
        let res = self.rhai_engine.call_fn_with_options::<T>(
            CallFnOptions::new().eval_ast(false),
            &mut rhai::Scope::new(),
            self.rhai_ast,
            name,
            args,
        );

        match res {
            Ok(res) => {
                Ok(res)
            },
            Err(err) => {
                warn!("call_function with {name} failed with {err}");
                Err(err.to_string())
            },
        }
    }
}