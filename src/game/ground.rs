use opengl_graphics::{ GlGraphics };
use graphics::*;
use rand::{ Rng, thread_rng };
use game::configure::{ CONTEXT, WINDOW_SIZE };
use game::render_info::RenderInfo;

//Ground

enum GroundType {
    Normal,
    NarrowWall,
    WideWall,
    Trap,
    Pit,
}

impl GroundType {
    fn get_height(&self) -> f64 {
        match *self {
            //GroundType::Normal    => CONTEXT.ground_height,
            GroundType::NarrowWall  => CONTEXT.ground_height * 3.0,
            GroundType::WideWall    => CONTEXT.ground_height * 3.0,
            _                       => CONTEXT.ground_height,
            //GroundType::Trap      => CONTEXT.ground_height,
            //GroundType::Pit       => CONTEXT.ground_height,
        }
    }

    fn get_width(&self) -> f64 {
        match *self {
            GroundType::NarrowWall  => CONTEXT.ground_width * 0.5,
            GroundType::WideWall    => CONTEXT.ground_width * 1.5,
            _                       => CONTEXT.ground_width,
        }
    }

    fn get_texture(&self) -> &str {
        match *self {
            GroundType::Normal  => "pic/ground.gif",
            GroundType::Trap    => "pic/trap.gif",
            GroundType::Pit     => "pic/pit.gif",
            _ => "pic/ground.gif",
        }
    }
}

fn get_type(dice: i32) -> GroundType {
    match dice {
        0...14  => GroundType::Normal,
        15...16 => GroundType::Trap,
        17      => GroundType::Pit,
        18      => GroundType::NarrowWall,
        19      => GroundType::WideWall,
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
        let ground_height = ground_type.get_height();
        let ground_width = ground_type.get_width();

        let initial_render_info: RenderInfo = RenderInfo::new([offset, WINDOW_SIZE[1] as f64 - ground_height, ground_width, ground_height], ground_type.get_texture());
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

    pub fn get_width(&self) -> f64 {
        self.render_info.get_width()
    }

    pub fn get_height(&self) -> f64 {
        self.render_info.get_height()
    }
}

