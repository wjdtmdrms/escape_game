use opengl_graphics::{ GlGraphics, Texture };
use graphics::*;
use std::path::Path;
use game::configure::{ CONTEXT, WINDOW_SIZE };
use game::render_info::RenderInfo;

// Runner
pub struct Runner {
    render_info: RenderInfo,

    // TODO: comment this
    fuel_accumulation_max: f64,
    fuel_accumulation_now: f64,
    fuel_consumption: f64,
    // TODO: comment this
    suit_durability_max: f64,
    suit_durability_now: f64,
    // TODO: comment this
    fusionable_material: f64,

    jump_count: i32,
    y_speed: f64, 

    lie_down_seq: i32, // for lie_down_animation.
    press_count: i32,
    accel_direction: f64,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            render_info: RenderInfo::new([CONTEXT.runner_init_x, CONTEXT.runner_init_y, CONTEXT.runner_width, CONTEXT.runner_height], Texture::from_path(Path::new("pic/daram.jpg")).unwrap()),
            fuel_accumulation_max: 0.0,
            fuel_accumulation_now: 0.0,
            fuel_consumption: 0.0,
            suit_durability_max: 0.0,
            suit_durability_now: 0.0,
            fusionable_material: 0.0,
            jump_count: 0,
            y_speed: 0.0,
            lie_down_seq: 0,
            press_count: 0,
            accel_direction: 0.0,
        }
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics) {
        self.render_info.render(c, gl);
    } 

    pub fn initiate_jump(&mut self) {
        if self.jump_count < CONTEXT.max_jump_cnt {
            self.jump_count += 1;
            self.y_speed = CONTEXT.runner_jump_speed;
        }
    }

    pub fn define_accel_direction(&mut self, dir: f64) {
        if dir == 1.0 || dir == -1.0 {
            if self.accel_direction != dir && self.press_count < 2 {
                self.accel_direction = dir;
                self.press_count += 1;
            }
        }
        else {
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
        //println!("Press Count: {}, Accel Dir: {}", self.press_count, self.accel_direction);
    }

    pub fn animate(&mut self, move_distance: f64, dt: f64) {
        // x, y moving.
        let dt_x: f64 = move_distance * self.accel_direction;
        let dt_y: f64 = self.y_speed * dt;

        self.render_info.mod_xy(dt_x, dt_y);

        if self.jump_count > 0 {
            self.y_speed += CONTEXT.gravity_accel * dt;
        }

        let limit_min_x = 0.0;
        let limit_max_x = (WINDOW_SIZE[0] as f64) - 1.5*CONTEXT.runner_width;
        let limit_max_y = CONTEXT.runner_init_y;

        self.render_info.limit_x(Some(limit_min_x), Some(limit_max_x));

        if self.render_info.limit_y(None, Some(limit_max_y)) {
            self.y_speed = 0.0;
            self.jump_count = 0;
        }
    }

    fn lie_down(&self) {

    }

    pub fn get_render_info(&self) -> &RenderInfo {
        &self.render_info
    }
}

