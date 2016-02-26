
// graphic related extern crates.
extern crate piston;
//extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

// graphic related uses.
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
//use graphics::*;

// mods.
mod game;

use game::Game;
use game::configure;
use game::input_manage;

// code begins.

fn main() {
    println!("Escape-Away Initiated !!");
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            configure::WINDOW_NAME,
            configure::WINDOW_SIZE
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
            game.press(input_manage::press_identifier(prs));
        }

        if let Some(rls) = e.release_args(){
            game.release(input_manage::release_identifier(rls));
        }
    }
}
