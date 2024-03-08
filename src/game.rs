pub mod actors;
pub mod input_parsing;
pub mod rooms;
use actors::{ai::*, organs::OrganPlugin};
use async_channel::{Receiver, Sender};
use bevy::{
    app::{AppExit, RunMode, ScheduleRunnerPlugin},
    prelude::*,
};
use std::time::Duration;

use self::{actors::player::PlayerActionEvent, input_parsing::parse_player_input};

const GAME_LOOP_MILIS: u64 = 100;

pub enum GameInputType {
    PlayerInput(String),
    Quit,
}

#[derive(Resource)]
struct GameInputReceiver(Receiver<GameInputType>);

#[derive(Resource)]
struct GameOutputSender(Sender<String>);

pub fn launch_game(rx: Receiver<GameInputType>, tx: Sender<String>) {
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

fn receive_input(
    rcv: Res<GameInputReceiver>,
    mut writer: EventWriter<PlayerActionEvent>,
    mut exit: EventWriter<AppExit>,
) {
    while let Ok(msg) = rcv.0.try_recv() {
        // parse message, send appropriate event
        // future versions will include the id of the sender, not just the message
        match msg {
            GameInputType::PlayerInput(input) => {
                let parsed = parse_player_input(&input);
                match parsed {
                    Ok(ev) => {
                        writer.send(ev);
                    }
                    Err(e) => {
                        // sends this error back to the bot
                        println!("{e}");
                    }
                }
            }
            GameInputType::Quit => {
                exit.send(AppExit);
            }
        }
    }
}
