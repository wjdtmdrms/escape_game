pub struct Context {
    pub land_width : f64,
    pub object_width : f64,
}

pub static CONTEXT: Context = Context {
    land_width : 128.0,
    object_width : 50.0
};

pub static WINDOW_NAME : &'static str = "Escape-Away";
pub static WINDOW_SIZE : [u32; 2] = [1280, 720];

pub static WHITE : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub static BROWN : [f32; 4] = [0.5, 0.25, 0.0, 1.0];
pub static RED : [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

