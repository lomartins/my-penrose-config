use penrose::{
    core::{
        data_types::Region,
        hooks::Hook,
        manager::WindowManager,
        xconnection::{XConn, Xid},
    },
    draw::{Color, DrawContext, Widget},
    Result, Selector,
};

pub struct StartupScript {
    path: String,
}
impl StartupScript {
    pub fn new(s: impl Into<String>) -> Self {
        Self { path: s.into() }
    }
}

impl<X: XConn> Hook<X> for StartupScript {
    fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
        spawn!(&self.path)
    }
}

pub struct CenterFloatTitle {
    title: String,
    scale: f64,
}

impl CenterFloatTitle {
    pub fn new(title: impl Into<String>, scale: f64) -> Box<Self> {
        Box::new(Self {
            title: title.into(),
            scale,
        })
    }

    fn centered_above<X: XConn>(&self, id: Xid, wm: &mut WindowManager<X>) -> Result<()> {
        if let Some(region) = wm.screen_size(wm.active_screen_index()) {
            let r = region.scale_w(self.scale).scale_h(self.scale);
            wm.position_client(id, r.centered_in(&region)?, true)?;
        }
        wm.show_client(id)
    }
}

impl<X: XConn> Hook<X> for CenterFloatTitle {
    fn new_client(&mut self, wm: &mut WindowManager<X>, id: u32) -> Result<()> {
        if let Some(c) = wm.client_mut(&id.into()) {
            if c.wm_name() == self.title {
                c.set_floating(true);
                self.centered_above(c.id(), wm)?;
            }
        }

        Ok(())
    }
}