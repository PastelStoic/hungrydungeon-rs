use crate::game::{
    actors::{organs::Organ, Actor},
    rooms::GameRoom,
};

use super::{
    parse_input::{parse_player_input, PlayerActionEventType},
    Player, PlayerAttackEvent, PlayerDevourEvent, PlayerMoveRoomEvent, PlayerStruggleEvent,
};
use bevy::prelude::*;

pub enum ParsedPlayerEvent {
    Attack(PlayerAttackEvent),
    Devour(PlayerDevourEvent),
    Move(PlayerMoveRoomEvent),
    Struggle(PlayerStruggleEvent),
    // other possible events: login, logout, useability,
    // move prey between organs, spit out prey, interact,
}

pub fn map_input_to_event(
    In((player_id, input)): In<(u64, String)>,
    q_actors: Query<(Entity, &Player)>,
    q_actor_names: Query<(Entity, &Name), With<Actor>>,
    q_organ_names: Query<(Entity, &Name), With<Organ>>,
    q_room_names: Query<(Entity, &Name), With<GameRoom>>,
) -> Result<ParsedPlayerEvent, String> {
    let player = q_actors.iter().find(|p| p.1 .0 == player_id);
    let Some(player) = player else {
        return Err(format!("Could not find player with id {player_id}"));
    };

    let player = player.0;

    match parse_player_input(&input) {
        Ok(ev_type) => match ev_type {
            PlayerActionEventType::Attack { target_name } => {
                let target = q_actor_names.iter().find(|e| e.1.as_str() == target_name);
                match target {
                    Some(target) => Ok(ParsedPlayerEvent::Attack(PlayerAttackEvent {
                        player,
                        target: target.0,
                    })),
                    None => Err(format!("Could not find actor named {target_name}")),
                }
            }
            PlayerActionEventType::Devour {
                target_name,
                organ_name,
            } => {
                let target = q_actor_names.iter().find(|e| e.1.as_str() == target_name);
                let organ = q_organ_names.iter().find(|e| e.1.as_str() == organ_name);
                let Some(target) = target else {
                    return Err(format!("Could not find actor named {target_name}"));
                };

                let Some(organ) = organ else {
                    return Err(format!("Could not find organ named {organ_name}"));
                };

                Ok(ParsedPlayerEvent::Devour(PlayerDevourEvent {
                    player,
                    prey: target.0,
                    organ: organ.0,
                }))
            }
            PlayerActionEventType::MoveRoom { room_name } => {
                let room = q_room_names.iter().find(|e| e.1.as_str() == room_name);
                match room {
                    Some(room) => Ok(ParsedPlayerEvent::Move(PlayerMoveRoomEvent {
                        player,
                        room: room.0,
                    })),
                    None => Err(format!("No room found called {room_name}")),
                }
            }
            PlayerActionEventType::Struggle => {
                Ok(ParsedPlayerEvent::Struggle(PlayerStruggleEvent { player }))
            }
        },
        Err(e) => Err(e.to_string()),
    }
}
