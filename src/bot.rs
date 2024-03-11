use async_channel::{Receiver, Sender};
use std::io::stdin;

use crate::game::{actors::player::PlayerInputStringEvent, GameInputType};

pub fn launch_bot(_rx: Receiver<String>, tx: Sender<GameInputType>) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            loop {
                let mut s = String::new();
                stdin().read_line(&mut s).expect("Invalid string input");

                if s == "exit" {
                    tx.send(GameInputType::Quit).await.unwrap();
                    break;
                }

                tx.send(GameInputType::PlayerInput(5, s)).await.unwrap();
            }
        });
}
