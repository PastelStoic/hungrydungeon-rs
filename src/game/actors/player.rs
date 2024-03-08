pub mod process_action;
use bevy::prelude::*;

use super::Actor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_systems(Update, player_attack);
    }
}

#[derive(Component)]
pub struct Player(pub u64);

fn player_attack(
    mut q_actors: Query<(Entity, &Name, &mut Actor)>,
    mut reader: EventReader<PlayerActionEvent>,
) {
}
