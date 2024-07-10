// #![allow(dead_code)]
mod collidable;
mod common;
mod data_transfer;
mod game;
mod snake;

use crate::data_transfer::{MessageProducer, MessageReciever};
use game::Game;
use serde_json;

fn main() {
    let mut game = Game::new(20, 20);
    println!(
        "{}",
        serde_json::to_string_pretty(&game.output_message()).unwrap()
    );
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        game.handle_input(input.trim().to_string());
        if game.is_running {
            game.tick()
        };
        println!("{:?}", game);
        println!(
            "{}",
            serde_json::to_string_pretty(&game.output_message()).unwrap()
        );
    }
}
