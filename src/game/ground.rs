
extern crate graphics;

extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use self::graphics::*;
use std::path::Path;

use super::configure;
// Ground

enum GroundPattern{
    
}

enum GroundType{
    Normal,
    Trap,
    Pit,
}

pub struct Ground{
	render_info : [f64; 4],
    g_type : GroundType,
    texture : Texture,
}

impl Ground{
    pub fn new(t : i32) -> Ground{
        //let dice : i32 = rand::thread_rng().gen_range(0, 10);
        let initial_render_info : [f64; 4] = [1280.0, 648.0, 128.0, 72.0];
        if t >= 0 && t < 17 {
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Normal,
                texture : Texture::from_path(Path::new("pic/ground.jpg")).unwrap(),
            }
        }
        else if t >= 17 && t < 19{
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Trap,
                texture : Texture::from_path(Path::new("pic/trap.jpg")).unwrap(),
            }
        }
        else{
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Pit,
                texture : Texture::from_path(Path::new("pic/pit.jpg")).unwrap(),
            }
        }
    }

    pub fn mod_xy(&mut self, dt_x : f64, dt_y : f64){
        self.render_info[0] += dt_x;
        self.render_info[1] += dt_y;
    }

    pub fn init_move(&mut self, i : i32){
        let width : f64 = self.render_info[2];
        self.mod_xy(-1.0 * width * ((10 - i) as f64), 0.0);
    }

    pub fn render(&self, c : Context, gl : &mut GlGraphics){
        let initial_render_info = [0.0, 0.0, self.render_info[2], self.render_info[3]];
        let image = Image::new().rect(initial_render_info);
        image.draw(&self.texture, default_draw_state(), c.transform.trans(self.render_info[0], self.render_info[1]), gl);
    }

    pub fn need_to_remove(&self) -> bool
    {
        if self.render_info[2] < -1.0 * self.render_info[0]{
            true
        }
        else{
            false
        }
    }

    pub fn animate(&mut self, speed : f64){
        self.render_info[0] -= speed;
    }
}

