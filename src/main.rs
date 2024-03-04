pub mod actors;
use actors::{ai::*, Actor};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<AttackActionEvent>()
        .insert_resource(AiTimer(Timer::from_seconds(10., TimerMode::Repeating)))
        .add_systems(Startup, spawn_test)
        .add_systems(Update, (tick_ai_timer, respond_to_ai, run_ai))
        .run();
}

#[derive(Resource)]
pub struct AiTimer(Timer);

fn tick_ai_timer(mut aitimer: ResMut<AiTimer>, time: Res<Time>) {
    aitimer.0.tick(time.delta());
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
        Ai,
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
