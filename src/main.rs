pub mod actors;
pub mod rooms;
use async_channel::{Receiver, Sender};
use std::{io::stdin, thread, time::Duration};

use actors::{ai::*, organs::OrganPlugin};
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
};

const GAME_LOOP_MILIS: u64 = 100;

fn main() {
    let (s_game, r_game) = async_channel::unbounded();
    let (s_bot, r_bot) = async_channel::unbounded();

    thread::scope(|s| {
        let game = thread::Builder::new()
            .name("Bevy".to_string())
            .spawn_scoped(s, || run_game(r_game, s_bot))
            .unwrap();

        let console = thread::Builder::new()
            .name("console".to_string())
            .spawn_scoped(s, || {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        loop {
                            let mut s = String::new();
                            stdin().read_line(&mut s).expect("Invalid string input");

                            if s == "exit" {
                                break;
                            }

                            s_game.send(s).await.unwrap();
                        }
                    });
            })
            .unwrap();

        game.join().unwrap();
        console.join().unwrap();
    });
}

#[derive(Resource)]
struct GameInputReceiver(Receiver<String>);

#[derive(Resource)]
struct GameOutputSender(Sender<String>);

fn run_game(rx: Receiver<String>, tx: Sender<String>) {
    App::new()
        .insert_resource(GameInputReceiver(rx))
        .insert_resource(GameOutputSender(tx))
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin {
                run_mode: RunMode::Loop {
                    wait: Some(Duration::from_millis(GAME_LOOP_MILIS)),
                },
            }),
            AiPlugin,
            OrganPlugin,
        ))
        .add_systems(Startup, spawn_test)
        .add_systems(Update, receive_input)
        .run();
}

fn spawn_test(mut commands: Commands) {
    commands.spawn(rooms::GameRoom).with_children(|mut room| {
        slime::spawn(&mut room);
        slimegirl::spawn(&mut room);
    });
}

fn receive_input(rs: Res<GameInputReceiver>) {
    while let Ok(msg) = rs.0.try_recv() {
        println!("Game received input {msg}");
        // parse message, send appropriate event
        // future versions will include the id of the sender, not just the message
    }
}
