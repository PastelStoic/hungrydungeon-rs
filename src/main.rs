pub mod actors;
use actors::{ai::*, Actor};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<AttackActionEvent>()
        .add_systems(Update, run_ai)
        .add_systems(Update, respond_to_ai)
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
        }
    ));
}