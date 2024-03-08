use bevy::prelude::*;

use crate::game::input_parsing::{self, parse_player_input};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerActionEvent>();
    }
}

#[derive(Event)]
pub enum PlayerActionEvent {
    Attack
}

pub fn react_to_player_input(msg: String) {
    let parsed = parse_player_input(&msg);
    match parsed {
        input_parsing::PlayerInputParseResult::Attack { target } => {
            println!("Received attack target {target}")
        }
        input_parsing::PlayerInputParseResult::Devour { target, organ } => {
            println!("Received devour {target} with organ {organ}")
        }
        input_parsing::PlayerInputParseResult::MoveRoom { room } => {
            println!("Received move to room {room}")
        }
        input_parsing::PlayerInputParseResult::Struggle => println!("Received struggle"),
        input_parsing::PlayerInputParseResult::Error(e) => println!("Parsing error: {e}"),
        input_parsing::PlayerInputParseResult::Unknown => println!("Received unknown input"),
    }
}