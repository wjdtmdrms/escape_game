
// Code containing input related functions.

extern crate piston;

use piston::input::*;

static mut does_spacekey_released : bool = true;

pub fn press_checker(btn : Button) -> bool{
	match btn{
		Button::Keyboard(Key::Space) => {
			if does_spacekey_released{
				println!("Space button pressed!! Jump!!");
				does_spacekey_released = false;
			    return true
			}
        }
		_ => {}
    }
    false
}

pub fn release_checker(btn : Button){
	match btn {
		Button::Keyboard(Key::Space) => {
			if !does_spacekey_released {
				println!("Space button released!!");
				does_spacekey_released = true;
			}
		}
		_ => {}
	}
}
