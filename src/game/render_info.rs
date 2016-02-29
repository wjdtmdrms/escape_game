use std::path::Path;
use opengl_graphics::{ GlGraphics, Texture };
use graphics::*;

pub fn is_crashed(it: &RenderInfo, other: &RenderInfo) -> bool {
    let i_x = it.get_offset_x();
    let i_y = it.get_offset_y();
    let o_x = other.get_offset_x();
    let o_y = other.get_offset_y();

    (it.is_include_x(o_x) || other.is_include_x(i_x))
        &&
        (it.is_include_y(o_y) || other.is_include_y(i_y))
}

pub struct RenderInfo {
    render_info: [f64; 4],
    texture: Texture,
}

impl RenderInfo {
    // pub fn new(x: f64, y: f64, w: f64, h: f64) -> RenderInfo {
    // RenderInfo {
    // render_info: [x, y, w, h],
    // }
    // }

    pub fn new(ren_info: [f64; 4], tx: &str) -> RenderInfo {
        RenderInfo {
            render_info: ren_info,
            texture: Texture::from_path(Path::new(tx)).unwrap(),
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let initial_render_info = [0.0, 0.0, self.render_info[2], self.render_info[3]];
        let image = Image::new().rect(initial_render_info);
        image.draw(&self.texture, default_draw_state(), c.transform.trans(self.render_info[0], self.render_info[1]), gl);
    }

    pub fn get_offset_x(&self) -> f64 {
        self.render_info[0]
    }

    pub fn get_offset_y(&self) -> f64 {
        self.render_info[1]
    }

    pub fn is_hidden_x(&self) -> bool {
        self.render_info[0] + self.render_info[2] < 0.0
    }

    pub fn is_include_x(&self, x: f64) -> bool {
        self.render_info[0] <= x && x < self.render_info[0] + self.render_info[2]
    }

    pub fn is_include_y(&self, y: f64) -> bool {
        self.render_info[1] <= y && y < self.render_info[1] + self.render_info[3]
    }

    pub fn mod_xy(&mut self, dt_x: f64, dt_y: f64) {
        self.render_info[0] += dt_x;
        self.render_info[1] += dt_y;
    }

    pub fn limit_x(&mut self, min_x: Option<f64>, max_x: Option<f64>) -> bool {
        let mut ret: bool = false;

        if let Some(x) = min_x {
            if self.render_info[0] < x {
                self.render_info[0] = x;
                ret = true;
            }
        } 

        if let Some(x) = max_x {
            if self.render_info[0] > x {
                self.render_info[0] = x;
                ret = true;
            }
        }

        ret
    }

    pub fn limit_y(&mut self, min_y: Option<f64>, max_y: Option<f64>) -> bool {
        let mut ret: bool = false;

        if let Some(y) = min_y {
            if self.render_info[1] < y {
                self.render_info[1] = y;
                ret = true;
            }
        } 

        if let Some(y) = max_y {
            if self.render_info[1] > y {
                self.render_info[1] = y;
                ret = true;
            }
        }

        ret
    }
}
