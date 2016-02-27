
extern crate graphics;

extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use self::graphics::*;
use std::path::Path;

use super::configure;
// Runner

pub struct Runner{
    render_info : [f64; 4],
    texture : Texture,
    fuel_accumulation_max : f64,
    fuel_accumulation_now : f64,
    fuel_consumption : f64,
    suit_durability_max : f64,
    suit_durability_now : f64,
    fusionable_material : f64,
    jump_count : i32, // first jump? or seceond jump?
    quarter_jump_speed : f64,
    jump_interval : i32,
    jump_seq : i32, // for jump_animation.
    lie_down_seq : i32, // for lie_down_animation.
    press_count : i32,
    accel_direction : f64,
}

impl Runner{
    pub fn new() -> Runner{
        Runner{
            render_info : [144.0, 465.0, 157.0, 183.0],
            texture : Texture::from_path(Path::new("pic/daram.jpg")).unwrap(),
            fuel_accumulation_max : 0.0,
            fuel_accumulation_now : 0.0,
            fuel_consumption : 0.0,
            suit_durability_max : 0.0,
            suit_durability_now : 0.0,
            fusionable_material : 0.0,
            jump_count : 0,
            quarter_jump_speed : 5.0,
            jump_interval : 10,
            jump_seq : 0,
            lie_down_seq : 0,
            press_count : 0,
            accel_direction : 0.0,
        }
    }

    pub fn render(&mut self, c : Context, gl : &mut GlGraphics){
        let initial_render_info = [0.0, 0.0, self.render_info[2], self.render_info[3]];
        let image = Image::new().rect(initial_render_info);
        image.draw(&self.texture, default_draw_state(), c.transform.trans(self.render_info[0], self.render_info[1]), gl);
    } 

    fn mod_xy(&mut self, dt_x : f64, dt_y : f64){
        self.render_info[0] += dt_x;
        self.render_info[1] += dt_y;
    }

    pub fn initiate_jump(&mut self){
        if self.jump_seq == 0{
            if self.jump_count == 0{
                self.jump_seq = self.jump_interval;
            }
            else if self.jump_count == 2{
            // do nothing yet.
            }
        }
    }

    pub fn define_accel_direction(&mut self, dir : f64){
        if dir == 1.0 || dir == -1.0 {
            if self.accel_direction != dir && self.press_count < 2 {
                self.accel_direction = dir;
                self.press_count += 1;
            }
        }
        else{
            if dir * self.accel_direction > 0.0 {
                if self.press_count == 2 {
                    self.accel_direction *= -1.0;
                }
                else {
                    self.accel_direction = 0.0;
                }
            }
            self.press_count -= 1;
        }
        //println!("Press Count : {}, Accel Dir : {}", self.press_count, self.accel_direction);
    }

	pub fn animate(&mut self, speed : f64){
        let mut dt_x : f64 = speed * self.accel_direction;
        let mut dt_y : f64 = 0.0;
        let quotient : i32 = self.jump_seq / self.jump_interval;
        let mut cofficient : f64 = 1.0;
        
        // x moving.
        let next_x = self.render_info[0] + dt_x;
        if next_x < 0.0 || next_x > (configure::WINDOW_SIZE[0] as f64) - 1.5*self.render_info[2]{
            dt_x = 0.0;
        }
        
        // y moving.
        if self.jump_seq == 8 * self.jump_interval{
            self.jump_seq = 0;
        }
        else{
            if self.jump_seq >= 0 && self.jump_seq < self.jump_interval{
                cofficient = 0.0;
            }
            else{
                if quotient < 4{
                    cofficient = (4 - (quotient % 4)) as f64 * -1.0;
                }
                else{
                    cofficient = (quotient % 4) as f64;
                }
                self.jump_seq += 1;
            }
            dt_y = self.quarter_jump_speed * cofficient;
        }
        // println!("dt_x : {}, dt_y : {}", dt_x, dt_y);
        self.mod_xy(dt_x, dt_y);
	}

    fn lie_down(&self){

    }

    pub fn get_render_info(&self) -> [f64; 4] {
        self.render_info
    }
}

