pub mod configure;
pub mod input_manage;

mod ground;
mod runner;
mod object;
mod render_info;

use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
use rand::{ Rng, thread_rng };

use game::configure:: {
    CONTEXT,
    WINDOW_SIZE,
};
use game::ground::Ground;
use game::runner::Runner;
use game::object::Object;
use game::render_info::RenderInfo;

pub struct Game {
    gl: GlGraphics,
    speed: f64,
    ground_q: Vec<Ground>, // further ground_queue.
    runner: Runner,
    object_q: Vec<Object>, // further object_queue.
    does_spacekey_released: bool
}

impl Game {
    pub fn new(opengl: OpenGL) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            speed: 500.0,
            ground_q: vec![],
            runner: Runner::new(),
            object_q: vec![],
            does_spacekey_released: true,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let tmp_ground_q = &self.ground_q;
        let tmp_runner = &self.runner;
        let tmp_object_q = &self.object_q;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(configure::WHITE, gl);
            tmp_runner.render(&c, gl);
            for g in tmp_ground_q {
                g.render(&c, gl);
            }
            for o in tmp_object_q {
                o.render(&c, gl);
            }
        })
    }

    fn update_grounds(&mut self, move_distance: f64) {
        // move grounds
        for gr in &mut self.ground_q {
            gr.animate(move_distance);
        } 

        // remove ground 
        self.ground_q.retain(|ref gr| !gr.need_to_remove());

        // get offset tiles
        let mut offset: f64 = 0.0;
        if let Some(gr) = self.ground_q.last() {
            offset = gr.get_offset_x() + CONTEXT.land_width;
        }

        // add new grounds
        while offset < WINDOW_SIZE[0] as f64 {
            self.ground_q.push(Ground::new(offset));
            offset += CONTEXT.land_width;
        }
    }

    fn update_objects(&mut self, move_distance: f64) {
        let dice: i32 = thread_rng().gen_range(0, 6);
        // move objects
        for obj in &mut self.object_q {
            obj.animate(move_distance);
        }

        // remove object
        let runner_render_info: &RenderInfo = self.runner.get_render_info();
        self.object_q.retain(|ref obj| !obj.need_to_remove(runner_render_info));

        // get offset tiles
        let mut offset: f64 = 0.0;
        if let Some(obj) = self.object_q.last() {
            offset = obj.get_offset_x() + 1.5 * CONTEXT.object_width;
        }

        // add new objects
        if dice == 0 && offset < WINDOW_SIZE[0] as f64 {
            //println!("Create Object: {}", offset);
            self.object_q.push(Object::new());
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let move_distance = self.speed * args.dt;
        self.update_grounds(move_distance);
        self.update_objects(move_distance);
        self.runner.animate(move_distance, args.dt);
    }

    pub fn press(&mut self, kr: input_manage::KeyResult) {
        match kr {
            input_manage::KeyResult::Jump => {
                self.runner.initiate_jump();
            }
            input_manage::KeyResult::MoveLeft => {
                self.runner.define_accel_direction(-1.0);
            }
            input_manage::KeyResult::MoveRight => {
                self.runner.define_accel_direction(1.0);
            }
            _ => {
                //do nothing yet
            }
        }
    }

    pub fn release(&mut self, kr: input_manage::KeyResult) {
        match kr {
            input_manage::KeyResult::Jump => {
                // do nothing
            }
            input_manage::KeyResult::MoveLeft => {
                self.runner.define_accel_direction(-2.0);
            }
            input_manage::KeyResult::MoveRight => {
                self.runner.define_accel_direction(2.0);
            }
            _ => {
                //do nothing
            }
        }
    }
}
