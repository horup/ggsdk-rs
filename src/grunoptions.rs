
pub struct GRunOptions {
    pub window_title: String,
    pub window_initial_pos:Option<(f32, f32)>,
    pub window_initial_size:Option<(f32, f32)>,
    pub window_initial_active:Option<bool>
}

impl Default for GRunOptions {
    fn default() -> Self {
        Self {
            window_title: "ggsdk App".to_string(),
            window_initial_pos:None,
            window_initial_active:None,
            window_initial_size: None,
        }
    }
}
