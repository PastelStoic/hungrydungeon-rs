
use std::io::stdin;
use async_channel::{Receiver, Sender};

pub fn launch_bot(rx: Receiver<String>, tx: Sender<String>) {
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

                tx.send(s).await.unwrap();
            }
        });
}