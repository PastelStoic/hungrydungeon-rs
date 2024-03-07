pub mod actors;
pub mod rooms;
use std::{io::stdin, sync::mpsc::{self, Receiver}, thread, time::Duration};

use actors::{ai::*, organs::OrganPlugin};
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let game = thread::spawn(|| run_game());

    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Invalid string input");
        println!("result was s");

        if s == "exit" {
            break;
        }
    }

    game.join().unwrap();
}

struct GameInputReceiver(Receiver<String>);

fn run_game(rx: Receiver<String>) {
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
        .insert_non_send_resource(GameInputReceiver(rx))
        .run();
}

fn spawn_test(mut commands: Commands) {
    commands.spawn(rooms::GameRoom).with_children(|mut room| {
        slime::spawn(&mut room);
        slimegirl::spawn(&mut room);
    });
}
