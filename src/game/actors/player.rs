use bevy::prelude::*;

use super::Actor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>();
    }
}

#[derive(Component)]
pub struct Player(pub u64);

#[derive(Event)]
pub struct PlayerActionEvent {
    pub player: Entity,
    pub event_type: PlayerActionEventType,
}

pub enum PlayerActionEventType {
    Attack { target: String },
    Devour { target: String, organ: String },
    MoveRoom { room: String },
    Struggle,
}

fn player_attack(q_actors: Query<(Entity, &Name, &Actor)>) {}
