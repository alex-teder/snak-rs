use crate::{collidable::Collidable, snake::Snake};
use rand::Rng;

#[derive(Debug)]
pub struct Game {
    pub height: i8,
    pub width: i8,
    pub is_running: bool,
    pub snakes: Vec<Snake>,
    pub players: Vec<String>,
    pub apple_position: Option<(i8, i8)>,
}

impl Game {
    pub fn new(height: i8, width: i8) -> Self {
        Game {
            height,
            width,
            snakes: vec![],
            players: vec![],
            is_running: false,
            apple_position: None,
        }
    }

    pub fn tick(&mut self) {
        // move all snakes forward
        for snake in self.snakes.iter_mut() {
            let is_apple_ahead = self.apple_position.is_some()
                && snake.peek().collides_with(&self.apple_position.unwrap());

            snake.move_forward(is_apple_ahead);
        }

        if self.check_apple_eaten() {
            self.respawn_apple();
        }

        // check if any snake should die
        let mut snakes_to_die: Vec<usize> = vec![];
        for (i, snake_i) in self.snakes.iter().enumerate() {
            if snake_i.check_collisions_with_self() {
                snakes_to_die.push(i);
            }

            for (j, snake_j) in self.snakes.iter().enumerate() {
                if i != j && !snakes_to_die.contains(&i) {
                    if snake_i.check_collisions(snake_j) {
                        snakes_to_die.push(i);
                    }
                }
            }
        }

        // kill em
        for snake_index in snakes_to_die {
            self.snakes.remove(snake_index);
            self.players.remove(snake_index);
        }

        if self.snakes.len() == 0 {
            self.stop();
        }
    }

    pub fn start(&mut self) {
        self.respawn_apple();
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn add_player(&mut self, id: String) {
        assert!(!self.players.contains(&id), "Same IDs for players!");

        let new_snake = Snake::default(self.snakes.len(), self.height, self.width);
        self.snakes.push(new_snake);
        self.players.push(id);
    }

    pub fn respawn_apple(&mut self) {
        loop {
            let new_position = self.get_random_position();
            if !self.snakes.iter().any(|snake| {
                snake.body.collides_with(&new_position)
                    && !snake
                        .prev_tail
                        .is_some_and(|tail| tail.collides_with(&new_position))
            }) {
                self.apple_position = Some(new_position);
                break;
            }
        }
    }

    fn check_apple_eaten(&self) -> bool {
        if self.apple_position.is_none() {
            return false;
        }

        self.snakes
            .iter()
            .any(|snake| snake.head().collides_with(&self.apple_position.unwrap()))
    }

    fn get_random_position(&self) -> (i8, i8) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..self.width), rng.gen_range(0..self.height))
    }
}
