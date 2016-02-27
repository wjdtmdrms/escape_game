
extern crate graphics;

extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics, OpenGL, Texture };
use self::graphics::*;
use std::path::Path;

use super::configure;
// Objects

enum ObjectType {
	Material { value: f64 },
	Obstacle { damage: f64 },
	Relic { name: String, price: f64 },
}

enum MaterialPattern {
	Single,
	Arch3,
	Arch5,
	Wave5,
	Circle6,
}

pub struct Object {
	render_info: [f64; 4],
	obj_type: ObjectType,
	is_chekced: bool,
}

impl Object {
	pub fn decide_type(&self){

	}

	pub fn decide_coin_pattern(){

	}
}

