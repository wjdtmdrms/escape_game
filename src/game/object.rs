use opengl_graphics::{ GlGraphics, Texture };
use graphics::*;
use rand::{ Rng, thread_rng };
use std::path::Path;
use game::configure::CONTEXT;

// Objects
enum ObjectType {
    Ore,
    Asteroid,
    Star,
}

impl ObjectType {
    fn get_texture(&self) -> Texture {
        match *self {
            ObjectType::Ore => Texture::from_path(Path::new("pic/ore.jpg")).unwrap(),
            ObjectType::Asteroid => Texture::from_path(Path::new("pic/asteroid.jpg")).unwrap(),
            ObjectType::Star => Texture::from_path(Path::new("pic/star.jpg")).unwrap(),
        }
    }
}

fn get_type(dice: i32) -> ObjectType {
    match dice {
        0...13  => ObjectType::Ore,
        14...18 => ObjectType::Asteroid,
        19      => ObjectType::Star,
        _       => ObjectType::Ore,
    }
}

enum MaterialPattern {
    Single,
    Arch3,
    Arch5,
    Wave5,
    Circle6,
}

pub struct Object {
    render_info: [f64; 4],
    obj_type: ObjectType,
    texture: Texture,
}

fn is_covered(f1: f64, f2: f64, f3: f64, f4: f64) -> bool {
    f1 < f4 && f2 > f3
}

impl Object {
    pub fn new() -> Object {
        let dice: i32 = thread_rng().gen_range(0, 20);
        let object_type = get_type(dice);
        let height: f64 = thread_rng().gen_range(144, 515) as f64;
        let initial_render_info: [f64; 4] = [1280.0, height, CONTEXT.object_width, 50.0];
        let img_texture = object_type.get_texture();

        Object {
            render_info: initial_render_info,
            obj_type: object_type,
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

    pub fn need_to_remove(&self, ri: [f64; 4]) -> bool {
        let self_left = self.render_info[0];
        let self_right = self_left + self.render_info[2];
        let self_top = self.render_info[1];
        let self_bottom = self_top + self.render_info[3];
        let ri_left = ri[0];
        let ri_right = ri_left + ri[2];
        let ri_top = ri[1];
        let ri_bottom = ri_top + ri[3];

        self_right < 0.0 ||
            is_covered(self_left, self_right, ri_left, ri_right) && 
            is_covered(self_top, self_bottom, ri_top, ri_bottom)

    }

    pub fn animate(&mut self, move_distance: f64) {
        self.render_info[0] -= move_distance;
    }

    pub fn get_offset_x(&self) -> f64 {
        self.render_info[0]
    }
}

