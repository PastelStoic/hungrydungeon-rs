use async_channel::{Receiver, Sender};
use std::io::stdin;

use crate::game::GameInputType;

pub fn launch_bot(rx: Receiver<String>, tx: Sender<GameInputType>) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let writer = tokio::spawn(write_loop(tx));
            let reader = tokio::spawn(read_loop(rx));
            let _  = reader.await;
            let _  = writer.await;
        });
}

async fn read_loop(rx: Receiver<String>) {
    // need to figure out how this exits
    loop {
        let msg = rx.recv().await;
        match msg {
            Ok(msg) => {
                println!("{msg}");
            }
            // error only happens when channel is empty and closed
            Err(_) => break,
        }
    }
}

async fn write_loop(tx: Sender<GameInputType>) {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Invalid string input");

        if s == "exit" {
            tx.close();
            break;
        }

        tx.send(GameInputType::PlayerInput(5, s)).await.unwrap();
    }
}
