use rhai::CallFnOptions;
use crate::GGEngine;

impl GGEngine {
    pub(crate) fn rhai_register_functions(&mut self) {
       /* let scripts = self.scripts.clone();
        self.rhai_engine.register_fn("load_script", move |x: &str| {
            scripts.borrow_mut().load(x, x);
        });*/
    }
    
    pub fn call_function(&self, name: &str, args: impl rhai::FuncArgs) {
        let _ = self.rhai_engine.call_fn_with_options::<()>(
            CallFnOptions::new().eval_ast(false),
            &mut rhai::Scope::new(),
            &self.rhai_ast,
            name,
            args,
        );
    }
}