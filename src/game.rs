pub mod actors;
pub mod connections;
pub mod decision_table;
pub mod rooms;

use actors::{ai::*, organs::OrganPlugin};
use async_channel::{Receiver, Sender, TryRecvError};
use bevy::{
    app::{AppExit, RunMode, ScheduleRunnerPlugin},
    prelude::*,
};
use std::time::Duration;

use self::{
    actors::{
        player::{Player, PlayerInputStringEvent, PlayerPlugin},
        Actor,
    },
    connections::ConnectionManager,
};

const GAME_LOOP_MILIS: u64 = 100;

pub enum GameInputType {
    PlayerInput(u64, String),
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
        .insert_resource(ConnectionManager::new())
        .add_event::<SendMessageToBotEvent>()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin {
                run_mode: RunMode::Loop {
                    wait: Some(Duration::from_millis(GAME_LOOP_MILIS)),
                },
            }),
            AiPlugin,
            OrganPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, spawn_test)
        .add_systems(Update, (receive_input, send_output))
        .run();
}

fn spawn_test(world: &mut World) {
    let room = world.spawn(rooms::GameRoom).id();

    let slime = slime::spawn(world);
    let slimegirl = slimegirl::Slimegirl::spawn(world);
    let player = world
        .spawn((Name::new("Player"), Player(5), Actor::default()))
        .id();

    world
        .entity_mut(room)
        .push_children(&[slime, slimegirl, player]);
}

/// Receives input from other threads and passes them to whichever system is meant to handle them.
fn receive_input(
    rcv: Res<GameInputReceiver>,
    snd: Res<GameOutputSender>,
    mut writer: EventWriter<PlayerInputStringEvent>,
    mut exit: EventWriter<AppExit>,
) {
    while let Ok(msg) = rcv.0.try_recv() {
        match msg {
            GameInputType::PlayerInput(id, input) => {
                writer.send(PlayerInputStringEvent(id, input));
            }
        }
    }
    if let Err(TryRecvError::Closed) = rcv.0.try_recv() {
        snd.0.close();
        exit.send(AppExit);
    }
}

fn send_output(snd: Res<GameOutputSender>, mut reader: EventReader<SendMessageToBotEvent>) {
    for ev in reader.read() {
        println!("{}", ev.message);
        // TODO note: below code does nothing while using stdin for input. Above line is needed for now.
        snd.0
            .try_send(ev.message.clone())
            .expect("Sending message to bot failed!");
    }
}
