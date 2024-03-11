mod parse_input;
mod process_input;
use bevy::prelude::*;
use process_input::{map_input_to_event, ParsedPlayerEvent};

use crate::game::SendMessageToBotEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputStringEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_event::<PlayerDevourEvent>()
            .add_event::<PlayerMoveRoomEvent>()
            .add_event::<PlayerStruggleEvent>()
            .add_systems(
                Update,
                (
                    process_event,
                    player_attack,
                    player_devour,
                    player_move_room,
                    player_struggle,
                ),
            );
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
        // this is a three-step process.
        // First, the string input is parsed to figure out what the player wants to do.
        // Second, the parsed event is checked to make sure the given names are valid entities.
        // Third, the event is passed to individual systems, which check whether the
        // named entities are what they're supposed to be and whether the action is possible.
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

fn player_attack() {}

fn player_devour() {}

fn player_move_room() {}

fn player_struggle() {}
