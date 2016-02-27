
extern crate graphics;
extern crate opengl_graphics;

// math related extern crates.
extern crate rand;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use self::graphics::*;
use std::path::Path;
//
// math related uses.
use self::rand::Rng;

use super::configure::CONTEXT;
// Ground

enum GroundPattern {

}

enum GroundType {
    Normal,
    Trap,
    Pit,
}

impl GroundType {
    fn get_height(&self) -> f64 {
        match *self {
            GroundType::Normal => 72.0,
            GroundType::Trap => 72.0,
            GroundType::Pit => 72.0,
        }
    }

    fn get_texture(&self) -> Texture {
        match *self {
            GroundType::Normal => Texture::from_path(Path::new("pic/ground.jpg")).unwrap(),
            GroundType::Trap => Texture::from_path(Path::new("pic/trap.jpg")).unwrap(),
            GroundType::Pit => Texture::from_path(Path::new("pic/pit.jpg")).unwrap(),
        }
    }
}

fn get_type(dice: i32) -> GroundType {
    match dice {
        0...16  => GroundType::Normal,
        17...18 => GroundType::Trap,
        19      => GroundType::Pit,
        _       => GroundType::Normal,
    }
}

pub struct Ground {
    render_info: [f64; 4],
    g_type: GroundType,
    texture: Texture,
}

impl Ground {
    pub fn new(offset: f64) -> Ground {
        let dice: i32 = rand::thread_rng().gen_range(0, 20);

        let ground_type = get_type(dice);

        let initial_render_info: [f64; 4] = [offset, 648.0, CONTEXT.land_width, 72.0];
        let img_texture = ground_type.get_texture();
        Ground {
            render_info: initial_render_info,
            g_type: ground_type,
            texture: img_texture,
        }
    }

    pub fn mod_xy(&mut self, dt_x: f64, dt_y: f64) {
        self.render_info[0] += dt_x;
        self.render_info[1] += dt_y;
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics) {
        let initial_render_info = [0.0, 0.0, self.render_info[2], self.render_info[3]];
        let image = Image::new().rect(initial_render_info);
        image.draw(&self.texture, default_draw_state(), c.transform.trans(self.render_info[0], self.render_info[1]), gl);
    }

    pub fn need_to_remove(&self) -> bool
    {
        self.render_info[2] < -1.0 * self.render_info[0]
    }

    pub fn animate(&mut self, move_distance: f64) {
        self.render_info[0] -= move_distance;
    }

    pub fn get_offset_x(&self) -> f64 {
        self.render_info[0]
    }
}

