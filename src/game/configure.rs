pub struct Context {
    // global config
    pub gravity_accel: f64,

    // ground config
    pub land_width: f64,
    pub land_height: f64,

    // object config 
    pub object_width: f64,
    pub object_height: f64,

    // runner config
    pub max_jump_cnt: i32,
    pub runner_width: f64,
    pub runner_height: f64,
    pub runner_init_x: f64,
    pub runner_init_y: f64,
    pub runner_jump_speed_1: f64,
    pub runner_jump_speed_2: f64,
}

pub static CONTEXT: Context = Context {
    // global config
    gravity_accel: 5000.0,

    // ground config
    land_width: 128.0,
    land_height: 72.0,

    // object config 
    object_width: 50.0,
    object_height: 50.0,

    // runner config
    max_jump_cnt: 2,
    runner_width: 157.0,
    runner_height: 183.0,
    runner_init_x: 144.0,
    runner_init_y: 464.0,
    runner_jump_speed_1: -1850.0,
    runner_jump_speed_2: -1250.0,
};

pub static WINDOW_NAME: &'static str = "Escape-Away";
pub static WINDOW_SIZE: [u32; 2] = [1280, 720];

pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub static BROWN: [f32; 4] = [0.5, 0.25, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

