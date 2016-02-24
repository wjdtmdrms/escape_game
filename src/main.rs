
// graphic related extern crates.
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics; // duplicated

// math related extern crates.
extern crate rand;

// graphic related uses.
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL }; // duplicated
use graphics::*;

// math related uses.
use rand:: Rng;

// mods.
mod component;
mod ui_manage;

// mod uses.
use component::Ground;
use component::Runner;
use component::Object;

struct Game{
    gl : GlGraphics,
    speed : f64,
    ground_q : Vec<Ground>, // further ground_queue.
    runner : Runner,
    //object : Object, // further object_queue.
    does_spacekey_released : bool
}

impl Game{
    fn new(opengl : OpenGL) -> Game{
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

    fn render(&mut self, args: &RenderArgs){
        let tmp_ground_q = &mut self.ground_q;
        let tmp_runner = &mut self.runner;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(component::WHITE, gl);
            tmp_runner.render(c, gl);
            for g in tmp_ground_q{
                g.render(c, gl);
            }
        })
    }

    fn move_grounds(&mut self){
        let gr_q = &mut self.ground_q;
        for mut gr in gr_q{
            gr.mod_xy(-1.0 * self.speed, 0.0);
        }
    }

    fn remove_grounds(&mut self){
        let gr_q = &mut self.ground_q;
        gr_q.retain(|ref gr| !gr.need_to_remove());
    }

    fn add_grounds(&mut self){
        let gr_q = &mut self.ground_q;
        let dice : i32 = rand::thread_rng().gen_range(0, 20);
        gr_q.push(Ground::new(dice));
    }

    fn renew_ground_q(&mut self){
        /*
        let new_ground_q : Vec<Ground> = self.ground_q.iter()
                                         .map(|&g| g.mod_xy(-1.0 * self.speed, 0.0))
                                         .collect::<Vec<Ground>>();
        */

        self.move_grounds();
        if self.ground_q.iter().any(|ref gr| gr.need_to_remove()){
            self.remove_grounds();
            self.add_grounds();
        }
        //println!("Ground_Q Length : {}", self.ground_q.len());
    }

    fn update(&mut self, args: &UpdateArgs){
        self.renew_ground_q();
        self.runner.animate_jump();
        self.runner.animate_moving(self.speed);
    }

    fn press(&mut self, btn : Button){
        match btn{
            Button::Keyboard(Key::Space) => {
                if self.does_spacekey_released{
                    println!("PRESS : Space");
                    self.does_spacekey_released = false;
                    self.runner.initiate_jump();
                }
            }
            Button::Keyboard(Key::Right) => {
                if self.runner.get_accelerating_direction() != 1.0 && self.runner.get_press_count() != 2{
                    println!("PRESS : Right");
                    self.runner.increase_press_count();
                    self.runner.set_accelerating_direction(1.0);
                }
            }
            Button::Keyboard(Key::Left) => {
                if self.runner.get_accelerating_direction() != -1.0 && self.runner.get_press_count() != 2{
                    println!("PRESS : Left");
                    self.runner.increase_press_count();
                    self.runner.set_accelerating_direction(-1.0);
                }
            }
            _ => {
                println!("Press : Other key");
            }
        }
    }

    fn release(&mut self, btn : Button){
        match btn{
            Button::Keyboard(Key::Space) => {
                if !self.does_spacekey_released{
                    println!("Release : Space");
                    self.does_spacekey_released = true;
                }
            }
            Button::Keyboard(Key::Right) => {
                println!("Release : Right");
                self.runner.decrease_press_count();
                if self.runner.get_press_count() == 1{
                    self.runner.set_accelerating_direction(-1.0);
                }
                else{
                    self.runner.set_accelerating_direction(0.0);
                }
            }
            Button::Keyboard(Key::Left) => {
                println!("Release : Left");
                self.runner.decrease_press_count();
                if self.runner.get_press_count() == 1{
                    self.runner.set_accelerating_direction(1.0);
                }
                else{
                    self.runner.set_accelerating_direction(0.0);
                }
            }
            _ => {
                println!("Release : Other Key");
            }
        }
    }
}
            
// code begins.

fn main() {
    println!("Escape-Away Initiated !!");
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            ui_manage::WINDOW_NAME,
            ui_manage::WINDOW_SIZE
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(opengl);
    
    let mut events = window.events();
    while let Some(e) = events.next(&mut window){
        if let Some(ren) = e.render_args(){
            game.render(&ren);
        }

        if let Some(upd) = e.update_args(){
            game.update(&upd);
        }

        if let Some(prs) = e.press_args(){
            game.press(prs);
        }

        if let Some(rls) = e.release_args(){
            game.release(rls);
        }
    }
}
