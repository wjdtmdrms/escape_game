use opengl_graphics::{ GlGraphics };
use graphics::*;
use rand::{ Rng, thread_rng };
use game::configure::{ CONTEXT, WINDOW_SIZE };
use game::render_info::RenderInfo;

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
            GroundType::Normal  => 72.0,
            GroundType::Trap    => 72.0,
            GroundType::Pit     => 72.0,
        }
    }

    fn get_texture(&self) -> &str {
        match *self {
            GroundType::Normal  => "pic/ground.gif",
            GroundType::Trap    => "pic/trap.gif",
            GroundType::Pit     => "pic/pit.gif",
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
    render_info: RenderInfo,
    g_type: GroundType,
}

impl Ground {
    pub fn new(offset: f64) -> Ground {
        let dice: i32 = thread_rng().gen_range(0, 20);

        let ground_type = get_type(dice);

        let initial_render_info: RenderInfo = RenderInfo::new([offset, WINDOW_SIZE[1] as f64 - CONTEXT.land_height, CONTEXT.land_width, CONTEXT.land_height], ground_type.get_texture());
        Ground {
            render_info: initial_render_info,
            g_type: ground_type,
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        self.render_info.render(c, gl);
    }

    pub fn need_to_remove(&self) -> bool {
        self.render_info.is_hidden_x()
    }

    pub fn animate(&mut self, move_distance: f64) {
        self.render_info.mod_xy(-move_distance, 0.0);
    }

    pub fn get_offset_x(&self) -> f64 {
        self.render_info.get_offset_x()
    }
}

