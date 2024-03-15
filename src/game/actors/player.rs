mod parse_input;
mod process_input;
use bevy::{
    ecs::system::{RunSystemOnce, SystemState},
    prelude::*,
};
use process_input::{map_input_to_event, ParsedPlayerEvent};

use crate::game::SendMessageToBotEvent;

use super::{organs::Organ, Actor};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputStringEvent>();
    }
}

#[derive(Event)]
pub struct PlayerInputStringEvent(pub u64, pub String);

pub struct PlayerAttackEvent {
    pub player: Entity,
    pub target: Entity,
}

pub struct PlayerDevourEvent {
    pub player: Entity,
    pub prey: Entity,
    pub organ: Entity,
}

pub struct PlayerMoveRoomEvent {
    pub player: Entity,
    pub room: Entity,
}

pub struct PlayerStruggleEvent {
    pub player: Entity,
}

#[derive(Component)]
pub struct Player(pub u64);

pub fn process_event(
    world: &mut World,
    params: &mut SystemState<EventReader<PlayerInputStringEvent>>,
) {
    // this is a three-step process.
    // First, the string input is parsed to figure out what the player wants to do.
    // Second, the parsed event is checked to make sure the given names are valid entities.
    // Third, the event is passed to individual systems, which check whether the
    // named entities are what they're supposed to be and whether the action is possible.
    let mut events = vec![];
    for ev in params.get(world).read() {
        events.push((ev.0, ev.1.clone()))
    }

    for ev in events {
        match world.run_system_once_with((ev.0, ev.1), map_input_to_event) {
            Ok(parseres) => match parseres {
                ParsedPlayerEvent::Attack(e) => {
                    world.run_system_once_with(e, player_attack);
                }
                ParsedPlayerEvent::Devour(e) => {
                    world.run_system_once_with(e, player_devour);
                }
                ParsedPlayerEvent::Move(e) => {
                    world.run_system_once_with(e, player_move_room);
                }
                ParsedPlayerEvent::Struggle(e) => {
                    world.run_system_once_with(e, player_struggle);
                }
            },
            Err(e) => {
                // send an event back to the discord bot to print it
                world.send_event(SendMessageToBotEvent { message: e });
            }
        }
    }
}

fn player_attack(In(ev): In<PlayerAttackEvent>, mut query: Query<(&Name, &mut Actor)>) {
    let actors = query.get_many_mut([ev.player, ev.target]);
    if let Ok([attacker, mut target]) = actors {
        // check if the slime is still active, if the target is still in reach, if its still alive
        // the "is this target valid" check should be the same code both here and above
        target.1.health_current -= attacker.1.attack;
        println!(
            "{} attacks {}, dealing {} damage!",
            attacker.0, target.0, attacker.1.attack
        );
    }
}

fn player_devour(
    In(ev): In<PlayerDevourEvent>,
    q_organs: Query<(Entity, &Organ)>,
    q_names: Query<&Name>,
    mut commands: Commands,
) {
    // do some calculation based on the stats of the parent to determine if success

    // sets parent of target to the organ
    let organ = q_organs
        .get(ev.organ)
        .expect("Organ has vanished before using!");
    commands.entity(ev.prey).set_parent(organ.0);
    let pred = q_names.get(ev.player).expect("Actor should have name!");
    let prey = q_names.get(ev.prey).expect("Actor should have name!");
    let organ_name = q_names.get(organ.0).expect("Organ should have name!");
    println!("{} devours {} with their {}!", pred, prey, organ_name);
}

fn player_move_room(In(ev): In<PlayerMoveRoomEvent>) {
    println!("Move event");
}

fn player_struggle(In(ev): In<PlayerStruggleEvent>) {
    println!("Struggle event");
}
