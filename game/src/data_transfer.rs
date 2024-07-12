use crate::{common::Direction, game::Game};
use serde::Serialize;
use std::collections::HashMap;

const ADD_PLAYER_COMMAND: &str = "add";
const START_GAME_COMMAND: &str = "start";
const CHANGE_DIRECTION_COMMAND: &str = "ch";
const QUIT_COMMAND: &str = "quit";

const COORD_DELIMITER: &str = "x";
const PREV_TAIL_SEPARATOR: &str = "+";

#[derive(Serialize)]
pub enum OutMessage {
    #[serde(rename = "init")]
    Init { field: String, player_count: usize },
    #[serde(rename = "state")]
    GameState {
        players: HashMap<String, String>,
        ap: String,
    },
}

pub trait MessageReciever {
    fn handle_input(&mut self, input: String);
}

impl MessageReciever for Game {
    fn handle_input(&mut self, input: String) {
        let string_parts: Vec<&str> = input.split(',').collect();
        match string_parts[0] {
            ADD_PLAYER_COMMAND => {
                if let Some(id) = string_parts.get(1) {
                    self.add_player(id.to_string());
                }
            }
            START_GAME_COMMAND => {
                self.start();
            }
            CHANGE_DIRECTION_COMMAND => {
                let id = string_parts.get(1).expect("Invalid arguments!");
                let direction = string_parts.get(2).expect("Invalid arguments!");
                let index = self
                    .players
                    .iter()
                    .position(|v| v == *id)
                    .expect("ID not found!");
                match *direction {
                    "up" => {
                        self.snakes[index].change_direction(Direction::Up);
                    }
                    "down" => {
                        self.snakes[index].change_direction(Direction::Down);
                    }
                    "left" => {
                        self.snakes[index].change_direction(Direction::Left);
                    }
                    "right" => {
                        self.snakes[index].change_direction(Direction::Right);
                    }
                    _ => (),
                }
            }
            QUIT_COMMAND => {
                let id = string_parts.get(1).expect("Invalid arguments!");
                let index = self
                    .players
                    .iter()
                    .position(|v| v == *id)
                    .expect("ID not found!");
                self.players.remove(index);
                self.snakes.remove(index);
            }
            _ => (),
        }
    }
}

pub trait MessageProducer {
    fn output_message(&self) -> OutMessage;
}

impl MessageProducer for Game {
    fn output_message(&self) -> OutMessage {
        if !self.is_running {
            return OutMessage::Init {
                field: format!("{}{}{}", self.width, COORD_DELIMITER, self.height),
                player_count: self.players.len(),
            };
        }

        let mut ap = String::new();
        if let Some((x, y)) = self.apple_position {
            ap = format!("{}{}{}", x, COORD_DELIMITER, y);
        }

        let mut players: HashMap<String, String> = HashMap::new();
        for (index, id) in self.players.iter().enumerate() {
            let mut result = vec![];
            for (x, y) in self.snakes[index].body.iter() {
                result.push(format!("{}{}{}", x, COORD_DELIMITER, y));
            }

            let mut result = result.join(",");
            if let Some((x, y)) = self.snakes[index].prev_tail {
                result = result + &format!("{}{}{}{}", PREV_TAIL_SEPARATOR, x, COORD_DELIMITER, y);
            }

            players.insert(id.to_string(), result);
        }

        return OutMessage::GameState { players, ap };
    }
}
