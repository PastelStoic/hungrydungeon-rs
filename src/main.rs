pub mod actors;
use actors::{
    ai::{self, *},
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
    commands.spawn((
        Name::new("Slime A"),
        Actor {
            health_current: 100,
            health_max: 100,
            attack: 10,
            defense: 10,
        },
        ai::slime::SlimeAi,
    ));
    commands.spawn((
        Name::new("Slime B"),
        Actor {
            health_current: 100,
            health_max: 100,
            attack: 10,
            defense: 10,
        },
    ));
}
