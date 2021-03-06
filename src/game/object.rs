use opengl_graphics::{ GlGraphics };
use graphics::*;
use rand::{ Rng, thread_rng };
use game::configure::{ CONTEXT, WINDOW_SIZE };
use game::render_info::{ RenderInfo, is_crashed };

// Objects
enum ObjectType {
    Ore,
    Asteroid,
    Star,
}

impl ObjectType {
    fn get_texture(&self) -> &str {
        match *self {
            ObjectType::Ore         => "pic/ore.gif",
            ObjectType::Asteroid    => "pic/asteroid.gif",
            ObjectType::Star        => "pic/star.gif",
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
    render_info: RenderInfo,
    obj_type: ObjectType,
}

impl Object {
    pub fn new() -> Object {
        let dice: i32 = thread_rng().gen_range(0, 20);
        let object_type = get_type(dice);
        let height: f64 = thread_rng().gen_range(144, 515) as f64;
        let initial_render_info: RenderInfo = RenderInfo::new([WINDOW_SIZE[0] as f64, height, CONTEXT.object_width, CONTEXT.object_height], object_type.get_texture());

        Object {
            render_info: initial_render_info,
            obj_type: object_type,
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        self.render_info.render(c, gl);
    }

    pub fn need_to_remove(&self, ri: &RenderInfo) -> bool {
        self.render_info.is_hidden_x() || is_crashed(&self.render_info, ri)
    }

    pub fn animate(&mut self, move_distance: f64) {
        self.render_info.mod_xy(-move_distance, 0.0);
    }

    pub fn get_offset_x(&self) -> f64 {
        self.render_info.get_offset_x()
    }
}

