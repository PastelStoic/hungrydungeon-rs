mod parse_input;
mod process_input;
use bevy::prelude::*;
use process_input::{map_input_to_event, ParsedPlayerEvent};

use crate::game::SendMessageToBotEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputStringEvent>()
            .add_systems(Update, process_event);
    }
}

#[derive(Event)]
pub struct PlayerInputStringEvent(pub u64, pub String);

#[derive(Event)]
pub struct PlayerAttackEvent {
    pub player: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct PlayerDevourEvent {
    pub player: Entity,
    pub prey: Entity,
    pub organ: Entity,
}

#[derive(Event)]
pub struct PlayerMoveRoomEvent {
    pub player: Entity,
    pub room: Entity,
}

#[derive(Event)]
pub struct PlayerStruggleEvent {
    pub player: Entity,
}

#[derive(Component)]
pub struct Player(pub u64);

fn process_event(
    q_actors: Query<(Entity, &Player)>,
    q_names: Query<(Entity, &Name)>,
    mut reader: EventReader<PlayerInputStringEvent>,
    mut w_attack: EventWriter<PlayerAttackEvent>,
    mut w_devour: EventWriter<PlayerDevourEvent>,
    mut w_move: EventWriter<PlayerMoveRoomEvent>,
    mut w_struggle: EventWriter<PlayerStruggleEvent>,
    mut w_err: EventWriter<SendMessageToBotEvent>,
) {
    for ev in reader.read() {
        match map_input_to_event(ev.0, &ev.1, &q_actors, &q_names) {
            Ok(parseres) => match parseres {
                ParsedPlayerEvent::Attack(e) => {
                    w_attack.send(e);
                }
                ParsedPlayerEvent::Devour(e) => {
                    w_devour.send(e);
                }
                ParsedPlayerEvent::Move(e) => {
                    w_move.send(e);
                }
                ParsedPlayerEvent::Struggle(e) => {
                    w_struggle.send(e);
                }
            },
            Err(e) => {
                // send an event back to the discord bot to print it
                w_err.send(SendMessageToBotEvent { message: e });
            }
        }
    }
}
