pub mod actors;
pub mod rooms;
use actors::{
    ai::{self, *},
    organs::Organ,
    Actor,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, AiPlugin))
        .add_systems(Startup, spawn_test)
        .run();
}

// todo: spawn in two slimes to attack stuff
fn spawn_test(mut commands: Commands) {
    commands.spawn(rooms::GameRoom).with_children(|room| {
        room.spawn((
            Name::new("Slime A"),
            Actor {
                health_current: 100,
                health_max: 100,
                attack: 10,
                defense: 10,
            },
            ai::slimegirl::SlimeGirlAi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Slimegirl stomach"),
                Organ {
                    health_current: 100,
                    health_max: 100,
                    attack: 10,
                    defense: 10,
                    capacity: 100,
                    fullness_current: 0,
                    organ_type: actors::organs::OrganType::Generic,
                },
            ));
        });
        room.spawn((
            Name::new("Slime B"),
            Actor {
                health_current: 100,
                health_max: 100,
                attack: 10,
                defense: 10,
            },
        ));
    });
}
