
use ggsdk::egui::{Align2, Color32, FontId, Id, LayerId, Pos2};
use ggsdk::{egui, glow, InitContext, UpdateContext};
use ggsdk::glow::HasContext as _;
use ggsdk::GGAtlas;


struct State {
    pub vertex_array: glow::VertexArray,
    pub program: glow::Program,
    pub angle:f32
}

struct App {
    state:Option<State>,
    iterations:u64
}
impl Default for App {
    fn default() -> Self {
        Self {
            iterations:0,
            state:None
        }
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: InitContext) {

        g.assets.load::<GGAtlas>("smilie_1x1.png", "smilie");
        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };
        let (vertex_shader_source, fragment_shader_source) = (
            r#"
                const vec2 verts[6] = vec2[6](
                    vec2(-1.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0),
                    vec2(1.0, -1.0),
                    vec2(1.0, 1.0),
                    vec2(-1.0, 1.0)
                );
                const vec4 colors[6] = vec4[6](
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec4(1.0, 1.0, 1.0, 1.0),
                    vec4(1.0, 1.0, 1.0, 1.0)
                );
                const vec2 tex[6] = vec2[6](
                    vec2(0.0, 0.0),
                    vec2(0.0, 1.0),
                    vec2(1.0, 1.0),
                    vec2(1.0, 1.0),
                    vec2(1.0, 0.0),
                    vec2(0.0, 0.0)
                );
                out vec4 v_color;
                out vec2 TexCoord;
                uniform float u_angle;
                void main() {
                    v_color = colors[gl_VertexID];
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    gl_Position.x *= cos(u_angle);
                    TexCoord = tex[gl_VertexID];
                }
            "#,
            r#"
                precision mediump float;
                in vec4 v_color;
                in vec2 TexCoord;
                out vec4 out_color;
                uniform sampler2D ourTexture;
                void main() {
                    out_color = v_color *  texture(ourTexture, TexCoord);
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

            self.state = Some(State {
                vertex_array,
                program,
                angle:0.0
            });
        }
    }

    fn update(&mut self, g: UpdateContext) {
        self.iterations += 1;
        if g.assets.pending() != 0 {
            return;
        }
       

        egui::panel::TopBottomPanel::top("top_panel").show(g.egui_ctx, |ui| {
            ui.label(format!("Iterations: {}", self.iterations));
        });
        egui::Window::new("Controls").show(g.egui_ctx, |ui|{
            if ui.button("Left").is_pointer_button_down_on() {
                self.state.as_mut().unwrap().angle -= g.dt;
            }
            if ui.button("Right").is_pointer_button_down_on() {
                self.state.as_mut().unwrap().angle += g.dt;
            }
        });


        let painter = g.egui_ctx.layer_painter(LayerId::new(egui::Order::Background, Id::new("painter")));
        painter.text(Pos2::new(16.0, 24.0), Align2::LEFT_CENTER, "GL Example", FontId::default(), Color32::WHITE);
    }

    fn paint_glow(&mut self, g:ggsdk::PaintGlowContext) {
        if g.assets.pending() != 0 {
            return;
        }
        let smilie_atlas = g.assets.get::<GGAtlas>("smilie").unwrap().texture_id();
        let painter = g.painter;
        let state = self.state.as_mut().unwrap();
        let gl = g.painter.gl();
        unsafe { 
            let texture = painter.texture(smilie_atlas).unwrap();
            gl.enable(glow::FRAMEBUFFER_SRGB);
            gl.use_program(Some(state.program.clone())); 
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.uniform_1_f32(
                gl.get_uniform_location(state.program, "u_angle").as_ref(),
                state.angle,
            );
            gl.bind_vertex_array(Some(state.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.disable(glow::FRAMEBUFFER_SRGB);
        };
    }
}

fn main() {
    ggsdk::GGEngine::run(App::default(), Default::default());
}
