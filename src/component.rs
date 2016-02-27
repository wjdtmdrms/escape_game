
//extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use graphics::*;
use std::path::Path;
use game::configure;

// Ground
fn xy_zero_vector(vec : [f64; 4]) -> [f64; 4]{
    [0.0, 0.0, vec[2], vec[3]]
}

/*
fn mod_xy(mut ri : [f64; 4], dt_x : f64, dt_y : f64) -> [f64; 4]{
    [ri[0] + dt_x, ri[1] + dt_y, ri[2], ri[3]]
}
*/

enum GroundPattern{
    
}

enum GroundType{
    Normal,
    Trap,
    Pit,
}

pub struct Ground{
	render_info : [f64; 4],
    g_type : GroundType,
    texture : Texture,
}

impl Ground{
    pub fn new(t : i32) -> Ground{
        //let dice : i32 = rand::thread_rng().gen_range(0, 10);
        let initial_render_info : [f64; 4] = [1280.0, 648.0, 128.0, 72.0];
        if t >= 0 && t < 17 {
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Normal,
                texture : Texture::from_path(Path::new("pic/ground.jpg")).unwrap(),
            }
        }
        else if t >= 17 && t < 19{
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Trap,
                texture : Texture::from_path(Path::new("pic/trap.jpg")).unwrap(),
            }
        }
        else{
            Ground{
                render_info : initial_render_info,
                g_type : GroundType::Pit,
                texture : Texture::from_path(Path::new("pic/pit.jpg")).unwrap(),
            }
        }
    }

    pub fn mod_xy(&mut self, dt_x : f64, dt_y : f64){
        self.render_info[0] += dt_x;
        self.render_info[1] += dt_y;
    }

    pub fn init_move(&mut self, i : i32){
        let width : f64 = self.render_info[2];
        self.mod_xy(-1.0 * width * ((10 - i) as f64), 0.0);
    }

    pub fn render(&self, c : Context, gl : &mut GlGraphics){
        let initial_render_info = [0.0, 0.0, self.render_info[2], self.render_info[3]];
        let image = Image::new().rect(initial_render_info);
        image.draw(&self.texture, default_draw_state(), c.transform.trans(self.render_info[0], self.render_info[1]), gl);
    }

    pub fn need_to_remove(&self) -> bool
    {
        if self.render_info[2] < -1.0 * self.render_info[0]{
            true
        }
        else{
            false
        }
    }
}

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
    accel_rate : f64,
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
            accel_rate : 5.0,
        }
    }

    pub fn render(&mut self, c : Context, gl : &mut GlGraphics){
        //self.animate_jump();
        //self.animate_moving();
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

	pub fn animate_jump(&mut self){
        let mut dt_y : f64 = 0.0;
        let quotient : i32 = self.jump_seq / self.jump_interval;
       let mut cofficient : f64 = 1.0;

        if self.jump_seq >= 0 && self.jump_seq < self.jump_interval{
            return;
        }
        
        if quotient < 4{
            cofficient = (4 - (quotient % 4)) as f64 * -1.0;
        }
        else{
            cofficient = (quotient % 4) as f64;
        }
        if self.jump_seq == 8 * self.jump_interval{
            self.jump_seq = 0;
            return;
        }
        dt_y = self.quarter_jump_speed * cofficient;
        //println!("jump_seq : {}, quotient : {}, cofficient : {}", self.jump_seq, quotient, cofficient);
        self.mod_xy(0.0, dt_y);
        self.jump_seq += 1;
	}
/*
    pub fn get_press_count(&self) -> i32{
        self.press_count
    }

    pub fn increase_press_count(&mut self){
        self.press_count += 1;
    }

    pub fn decrease_press_count(&mut self){
        self.press_count -= 1;
    }

    pub fn get_accel_direction(&self) -> f64{
        self.accel_direction
    }

    pub fn set_accel_direction(&mut self, dir : f64){
        println!("Acc_Dir : {}", dir);
        self.accel_direction = dir;
    }
*/
    pub fn define_accel_direction(&mut self, dir : f64){
        if dir == 1.0 || dir == -1.0 {
            if self.accel_direction != dir && self.press_count < 2 {
                self.accel_direction = dir;
                self.press_count += 1;
            }
        }
        else{
            println!(" {} ", dir * self.accel_direction);
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
        println!("Press Count : {}, Accel Dir : {}", self.press_count, self.accel_direction);
    }

    pub fn animate_moving(&mut self){
        let dt_x : f64 = self.accel_rate * self.accel_direction;
        let next_x = self.render_info[0] + dt_x;
        if dt_x != 0.0 && next_x >= 0.0 && next_x <= (configure::WINDOW_SIZE[0] as f64) - 1.5*self.render_info[2]{
        self.mod_xy(dt_x, 0.0);
        }
    }

    fn lie_down(&self){

    }

    fn accelerate(&self){

    }

    fn decelerate(&self){

    }
}

// Objects

enum ObjectType{
	Material { value : f64 },
	Obstacle { damage : f64 },
	Relic{ name : String, price : f64 },
}

enum MaterialPattern{
	Single,
	Arch3,
	Arch5,
	Wave5,
	Circle6,
}

pub struct Object{
	render_info : [f64; 4],
	obj_type : ObjectType,
	is_chekced : bool,
}

impl Object{
	pub fn decide_type(&self){

	}

	pub fn decide_coin_pattern(){

	}
}

