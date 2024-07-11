// #![allow(dead_code)]
mod collidable;
mod common;
mod config;
mod data_transfer;
mod game;
mod snake;

use std::{
    env, process,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::data_transfer::{MessageProducer, MessageReciever};
use config::{create_config_from_args, CliError};
use game::Game;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let maybe_config = create_config_from_args(args);
    if let Err(e) = maybe_config {
        match e {
            CliError::Help => {
                println!("Options:");
                println!("");
                println!("  -h <HEIGHT>           Set the height of the field.");
                println!("  -w <WIDTH>            Set the width of the field.");
                println!("  -s <SIZE>             Set both the width and height of the field to the same value.");
                println!("  -t <TICK_INTERVAL>    Set the game tick interval in milliseconds.");
                println!("  --help                Show this help message and exit.");
                println!("");
                process::exit(0);
            }
            CliError::WrongInput => {
                println!("Incorrect arguments!");
                process::exit(1);
            }
        }
    }

    let config = maybe_config.ok().unwrap();

    let game = Arc::new(Mutex::new(Game::new(
        config.field_height,
        config.field_width,
    )));

    let game_tick = Arc::clone(&game);
    let tick_thread = thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            if last_tick.elapsed() >= config.tick_interval {
                let mut game = game_tick.lock().unwrap();
                if game.is_running {
                    game.tick();
                    println!("{}", serde_json::to_string(&game.output_message()).unwrap());
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
        println!("{}", serde_json::to_string(&game.output_message()).unwrap());
    });

    tick_thread.join().unwrap();
    input_thread.join().unwrap();
}
