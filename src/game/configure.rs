pub struct Context {
    pub land_width: f64,
    pub object_width: f64,
    pub runner_width: f64,
    pub runner_y_offset: f64,
    pub runner_jump_speed: f64,
    pub gravity_accel: f64,
}

pub static CONTEXT: Context = Context {
    land_width: 128.0,
    object_width: 50.0,
    runner_width: 157.0,
    runner_y_offset: 465.0,
    runner_jump_speed: -450.0,
    gravity_accel: 500.0,
};

pub static WINDOW_NAME: &'static str = "Escape-Away";
pub static WINDOW_SIZE: [u32; 2] = [1280, 720];

pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub static BROWN: [f32; 4] = [0.5, 0.25, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

