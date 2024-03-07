pub mod actors;
pub mod rooms;
use std::time::Duration;

use actors::{
    ai::{self, *},
    organs::{Organ, OrganPlugin},
    Actor,
};
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin {
                run_mode: RunMode::Loop {
                    wait: Some(Duration::from_millis(100)),
                },
            }),
            AiPlugin,
            OrganPlugin,
        ))
        .add_systems(Startup, spawn_test)
        .run();
}

fn spawn_test(mut commands: Commands) {
    commands.spawn(rooms::GameRoom).with_children(|mut room| {
        slime::spawn(&mut room);
        slimegirl::spawn(&mut room);
    });
}
