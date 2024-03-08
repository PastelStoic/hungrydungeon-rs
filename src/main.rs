pub mod bot;
pub mod game;
use std::thread;

use bot::launch_bot;

fn main() {
    let (s_game, r_game) = async_channel::unbounded();
    let (s_bot, r_bot) = async_channel::unbounded();

    thread::scope(|s| {
        let game = thread::Builder::new()
            .name("Bevy".to_string())
            .spawn_scoped(s, || game::launch_game(r_game, s_bot))
            .unwrap();

        let bot = thread::Builder::new()
            .name("console".to_string())
            .spawn_scoped(s, || launch_bot(r_bot, s_game))
            .unwrap();

        game.join().unwrap();
        bot.join().unwrap();
    });
}
