pub mod actors;
pub mod rooms;
use actors::{ai::*, organs::OrganPlugin};
use async_channel::{Receiver, Sender};
use bevy::{
    app::{AppExit, RunMode, ScheduleRunnerPlugin},
    prelude::*,
};
use std::time::Duration;

use self::actors::player::PlayerInputStringEvent;

const GAME_LOOP_MILIS: u64 = 100;

pub enum GameInputType {
    PlayerInput(u64, String),
    Quit,
}

#[derive(Event)]
pub struct SendMessageToBotEvent {
    message: String,
}

#[derive(Resource)]
struct GameInputReceiver(Receiver<GameInputType>);

#[derive(Resource)]
struct GameOutputSender(Sender<String>);

pub fn launch_game(rx: Receiver<GameInputType>, tx: Sender<String>) {
    App::new()
        .insert_resource(GameInputReceiver(rx))
        .insert_resource(GameOutputSender(tx))
        .add_event::<SendMessageToBotEvent>()
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
        .add_systems(Update, (receive_input, send_output))
        .run();
}

fn spawn_test(mut commands: Commands) {
    commands.spawn(rooms::GameRoom).with_children(|mut room| {
        slime::spawn(&mut room);
        slimegirl::spawn(&mut room);
    });
}

/// Receives input from other threads and passes them to whichever system is meant to handle them.
fn receive_input(
    rcv: Res<GameInputReceiver>,
    mut writer: EventWriter<PlayerInputStringEvent>,
    mut exit: EventWriter<AppExit>,
) {
    while let Ok(msg) = rcv.0.try_recv() {
        match msg {
            GameInputType::PlayerInput(id, input) => {
                writer.send(PlayerInputStringEvent(id, input));
            }
            GameInputType::Quit => {
                exit.send(AppExit);
            }
        }
    }
}

fn send_output(snd: Res<GameOutputSender>, mut reader: EventReader<SendMessageToBotEvent>) {
    for ev in reader.read() {
        snd.0
            .try_send(ev.message.clone())
            .expect("Sending message to bot failed!");
    }
}
