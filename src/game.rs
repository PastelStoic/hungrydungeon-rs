pub mod actors;
pub mod input_parsing;
pub mod rooms;
use actors::{ai::*, organs::OrganPlugin};
use async_channel::{Receiver, Sender};
use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
};
use std::time::Duration;

use self::actors::player::{react_to_player_input, PlayerActionEvent};

const GAME_LOOP_MILIS: u64 = 100;

pub enum GameInputType {
    PlayerInput(String),
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

fn receive_input(rs: Res<GameInputReceiver>, mut writer: EventWriter<PlayerActionEvent>) {
    while let Ok(msg) = rs.0.try_recv() {
        // parse message, send appropriate event
        // future versions will include the id of the sender, not just the message
        match msg {
            GameInputType::PlayerInput(msg) => react_to_player_input(msg),
        }
    }
}
