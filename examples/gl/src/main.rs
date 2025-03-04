use std::sync::Arc;
use std::sync::RwLock;

use ggsdk::egui;
use ggsdk::egui::LayerId;
use ggsdk::glow;
use ggsdk::glow::HasContext as _;


struct State {
    pub vertex_array: glow::VertexArray,
    pub program: glow::Program,
    pub angle:f32
}

struct App {
    state:Arc<RwLock<Option<State>>>
}
impl Default for App {
    fn default() -> Self {
        Self {
            state:Arc::new(RwLock::new(None))
        }
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: &mut ggsdk::GGContext) {
        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };
        let (vertex_shader_source, fragment_shader_source) = (
            r#"
                const vec2 verts[3] = vec2[3](
                    vec2(0.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0)
                );
                const vec4 colors[3] = vec4[3](
                    vec4(1.0, 0.0, 0.0, 1.0),
                    vec4(0.0, 1.0, 0.0, 1.0),
                    vec4(0.0, 0.0, 1.0, 1.0)
                );
                out vec4 v_color;
                uniform float u_angle;
                void main() {
                    v_color = colors[gl_VertexID];
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    gl_Position.x *= cos(u_angle);
                }
            "#,
            r#"
                precision mediump float;
                in vec4 v_color;
                out vec4 out_color;
                void main() {
                    out_color = v_color;
                }
            "#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        unsafe {
            let gl = g.gl;
            let program = gl.create_program().expect("Cannot create program");
            let _: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);

            let vertex_array = gl.create_vertex_array().expect("failed to create");

            *self.state.write().unwrap() = Some(State {
                vertex_array,
                program,
                angle:0.0
            });
        }
    }

    fn update(&mut self, g: &mut ggsdk::GGContext) {
        let screen_rect = g.egui_ctx.screen_rect();
        let state = self.state.clone();
        let callback = egui::PaintCallback {
            rect: screen_rect,
            callback: std::sync::Arc::new(ggsdk::egui_glow::CallbackFn::new(move |_info, painter| {
                let gl = painter.gl();
                let state = state.read().unwrap();
                let state = state.as_ref().unwrap();
                unsafe { 
                    gl.use_program(Some(state.program.clone())); 
                    gl.uniform_1_f32(
                        gl.get_uniform_location(state.program, "u_angle").as_ref(),
                        state.angle,
                    );
                    gl.bind_vertex_array(Some(state.vertex_array));
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                };
            })),
        };
        let mut state = self.state.write().unwrap();
        let state = state.as_mut().unwrap();
        state.angle += g.dt;
        g.egui_ctx
            .layer_painter(LayerId::background())
            .add(callback);
    }
}

fn main() {
    ggsdk::GGEngine::run(App::default(), Default::default());
}
