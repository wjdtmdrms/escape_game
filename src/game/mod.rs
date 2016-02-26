
// graphic related extern crates.
//extern crate piston;
extern crate graphics;
//extern crate glutin_window;
extern crate opengl_graphics;

// math related extern crates.
extern crate rand;

// graphic related uses.
//use piston::window::WindowSettings;
//use piston::event_loop::*;
use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use self::graphics::*;

// math related uses.
use self::rand::Rng;

// mod uses.
pub mod configure;
pub mod input_manage;
mod ground;
mod runner;
mod object;

use self::ground::Ground;
use self::runner::Runner;
use self::object::Object;

pub struct Game{
    gl : GlGraphics,
    speed : f64,
    ground_q : Vec<Ground>, // further ground_queue.
    runner : Runner,
    //object : Object, // further object_queue.
    does_spacekey_released : bool
}

impl Game{
    pub fn new(opengl : OpenGL) -> Game{
        let mut tmp_game = Game{
            gl : GlGraphics::new(opengl),
            speed : 5.0,
            ground_q : vec![],
            runner : Runner::new(),
        
            does_spacekey_released : true,
        };

        for i in 0..10 {
            let mut new_ground = Ground::new(0);
            new_ground.init_move(i);
            tmp_game.ground_q.push(new_ground);
        }
        tmp_game
    }

    pub fn render(&mut self, args: &RenderArgs){
        let tmp_ground_q = &mut self.ground_q;
        let tmp_runner = &mut self.runner;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(configure::WHITE, gl);
            tmp_runner.render(c, gl);
            for g in tmp_ground_q{
                g.render(c, gl);
            }
        })
    }

    fn renew_ground_q(&mut self){
        let dice : i32 = rand::thread_rng().gen_range(0, 20);
        {
            let gr_q = &mut self.ground_q;
            for mut gr in gr_q{
                gr.animate(self.speed);
            }
        }
        let gr_q = &mut self.ground_q;
        if gr_q.iter().any(|ref gr| gr.need_to_remove()) {
            gr_q.retain(|ref gr| !gr.need_to_remove());
            gr_q.push(Ground::new(dice));
        }
    }

    pub fn update(&mut self, args: &UpdateArgs){
        self.renew_ground_q();
        self.runner.animate(self.speed);
    }

    pub fn press(&mut self, kr : input_manage::KeyResult){
        match kr{
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

    pub fn release(&mut self, kr : input_manage::KeyResult){
        match kr{
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
