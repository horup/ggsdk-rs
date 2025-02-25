use crate::Action;

#[derive(Clone, Copy)]
pub enum FadeDirection {
    In,
    Out,
}

pub struct FadeAction {
    pub dir: FadeDirection,
}

impl FadeAction {
    pub fn fade_in() -> Self {
        Self { dir: FadeDirection::In }
    }
    pub fn fade_out() -> Self { 
        Self { dir: FadeDirection::Out }
    }
}

impl Action for FadeAction {
    fn exec(self:Box<Self>, ctx: &mut crate::ActionContext) {
        let dir: f32 = match self.dir {
            FadeDirection::In => -1.0,
            FadeDirection::Out => 1.0,
        };
        let speed = 3.0;
        let delta = ctx.dt * dir * speed;
        ctx.state.fade += delta;
        ctx.state.fade = ctx.state.fade.clamp(0.0, 1.0);
        match self.dir {
            FadeDirection::In => {
                if ctx.state.fade != 0.0 {
                    ctx.push_action(FadeAction {
                        dir: self.dir,
                    });
                }
            },
            FadeDirection::Out => {
                if ctx.state.fade != 1.0 {
                    ctx.push_action(FadeAction {
                        dir: self.dir,
                    });
                }
            },
        }
    }
}
