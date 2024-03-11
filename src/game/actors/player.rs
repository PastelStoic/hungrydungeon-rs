pub mod parse_input;
use bevy::prelude::*;

use self::parse_input::parse_player_input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputStringEvent>()
            .add_systems(Update, process_event);
    }
}

#[derive(Event)]
pub struct PlayerInputStringEvent(pub u64, pub String);

#[derive(Component)]
pub struct Player(pub u64);

fn process_event(
    q_actors: Query<(Entity, &Player)>,
    mut reader: EventReader<PlayerInputStringEvent>,
) {
    for ev in reader.read() {
        let player = q_actors.iter().find(|p| p.1 .0 == ev.0);
        let Some(_player) = player else {
            // log error
            continue;
        };

        let ev_type = parse_player_input(&ev.1);
        match ev_type {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
