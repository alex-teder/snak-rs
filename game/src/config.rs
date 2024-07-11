use std::{ops::RangeInclusive, time::Duration};

const DEFAULT_TICK_RATE: Duration = Duration::from_millis(500);
const DEFAULT_FIELD_SIZE: i8 = 20;
const FIELD_SIZE_RANGE: RangeInclusive<i8> = 5..=100;
const TICK_INTERVAL_MS_RANGE: RangeInclusive<u64> = 100..=10000;

pub struct GameConfig {
    pub field_width: i8,
    pub field_height: i8,
    pub tick_interval: Duration,
}

pub enum CliError {
    Help,
    WrongInput,
}

pub fn create_config_from_args(args: Vec<String>) -> Result<GameConfig, CliError> {
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
