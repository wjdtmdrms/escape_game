
// Code containing input related functions.

extern crate piston;

use piston::input::*;

pub enum KeyResult {
    None,
    // control
    Jump,
    MoveLeft,
    MoveRight,
    Crouch,
    // home menu
    NewGame,
    HowToPlay,
    Ranking,
    Exit,
    // game menu
    Pause,
    RestartGame,
    Home,
}

pub fn press_identifier(btn : Button) -> KeyResult {
    match btn {
        Button::Keyboard(Key::LCtrl) => {
            println!("PRESS : LCtrl");
            KeyResult::Jump
        }
        Button::Keyboard(Key::Left) => {
            println!("PRESS : Left");
            KeyResult::MoveLeft
        }
        Button::Keyboard(Key::Right) => {
            println!("PRESS : Right");
            KeyResult::MoveRight
        }
        Button::Keyboard(Key::Execute) => {
            println!("PRESS : Enter");
            KeyResult::NewGame
        }
        _ => {
            println!("PRESS : Other key");
            KeyResult::None
        }
    }
}

pub fn release_identifier(btn : Button) -> KeyResult {
	match btn {
        Button::Keyboard(Key::LCtrl) => {
            println!("Release : LCtrl");
            KeyResult::Jump
        }
    	Button::Keyboard(Key::Left) => {
			println!("Release : Left");
            KeyResult::MoveLeft
        }
        Button::Keyboard(Key::Right) => {
            println!("Release : Right");
            KeyResult::MoveRight
        }
        _ => {
            println!("Rlease : Other Key");
            KeyResult::None
        }
    }
}
