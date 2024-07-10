// #![allow(dead_code)]
mod collidable;
mod common;
mod data_transfer;
mod game;
mod snake;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::data_transfer::{MessageProducer, MessageReciever};
use game::Game;
use serde_json;

const TICK_RATE: Duration = Duration::from_millis(1500);

fn main() {
    let game = Arc::new(Mutex::new(Game::new(20, 20)));

    let game_tick = Arc::clone(&game);
    let tick_thread = thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            if last_tick.elapsed() >= TICK_RATE {
                let mut game = game_tick.lock().unwrap();
                if game.is_running {
                    game.tick();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&game.output_message()).unwrap()
                    );
                }
                last_tick = Instant::now();
            }
            thread::sleep(Duration::from_millis(10)); // Sleep to prevent busy waiting
        }
    });

    let game_input = Arc::clone(&game);
    let input_thread = thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut game = game_input.lock().unwrap();
        game.handle_input(input.trim().to_string());
        println!(
            "{}",
            serde_json::to_string_pretty(&game.output_message()).unwrap()
        );
    });

    tick_thread.join().unwrap();
    input_thread.join().unwrap();
}
