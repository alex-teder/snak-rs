// #![allow(dead_code)]
mod collidable;
mod common;
mod data_transfer;
mod game;
mod snake;

use std::{
    env,
    ops::RangeInclusive,
    process,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::data_transfer::{MessageProducer, MessageReciever};
use game::Game;
use serde_json;

const DEFAULT_TICK_RATE: Duration = Duration::from_millis(500);
const DEFAULT_FIELD_SIZE: i8 = 20;
const FIELD_SIZE_RANGE: RangeInclusive<i8> = 5..=100;
const TICK_INTERVAL_MS_RANGE: RangeInclusive<u64> = 100..=10000;

struct GameConfig {
    field_width: i8,
    field_height: i8,
    tick_interval: Duration,
}

enum CliError {
    Help,
    WrongInput,
}

fn create_config_from_args(args: Vec<String>) -> Result<GameConfig, CliError> {
    if args.contains(&"--help".to_string()) {
        return Err(CliError::Help);
    }

    let mut field_width: i8 = DEFAULT_FIELD_SIZE;
    let mut field_height: i8 = DEFAULT_FIELD_SIZE;
    let mut tick_interval: Duration = DEFAULT_TICK_RATE;

    let mut args_iter = args.iter().peekable();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-h" => {
                if let Some(value) = args_iter.next() {
                    if let Ok(num) = value.parse::<i8>() {
                        if FIELD_SIZE_RANGE.contains(&num) {
                            field_height = num;
                        } else {
                            return Err(CliError::WrongInput);
                        }
                    }
                }
            }

            "-w" => {
                if let Some(value) = args_iter.next() {
                    if let Ok(num) = value.parse::<i8>() {
                        if FIELD_SIZE_RANGE.contains(&num) {
                            field_width = num;
                        } else {
                            return Err(CliError::WrongInput);
                        }
                    }
                }
            }

            "-s" => {
                if let Some(value) = args_iter.next() {
                    if let Ok(num) = value.parse::<i8>() {
                        if FIELD_SIZE_RANGE.contains(&num) {
                            field_height = num;
                            field_width = num;
                        } else {
                            return Err(CliError::WrongInput);
                        }
                    }
                }
            }

            "-t" => {
                if let Some(value) = args_iter.next() {
                    if let Ok(num) = value.parse::<u64>() {
                        if TICK_INTERVAL_MS_RANGE.contains(&num) {
                            tick_interval = Duration::from_millis(num);
                        } else {
                            return Err(CliError::WrongInput);
                        }
                    }
                }
            }
            _ => return Err(CliError::WrongInput),
        }
    }

    return Ok(GameConfig {
        field_width,
        field_height,
        tick_interval,
    });
}

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
