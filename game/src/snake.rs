use crate::{collidable::Collidable, common::Direction};
use core::panic;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Snake {
    pub body: LinkedList<(i8, i8)>,
    pub current_direction: Direction,
    pub prev_tail: Option<(i8, i8)>,
    direction_locked: bool,
    field_width: i8,
    field_height: i8,
}

impl Snake {
    fn new(
        body: LinkedList<(i8, i8)>,
        direction: Direction,
        field_height: i8,
        field_width: i8,
    ) -> Self {
        Snake {
            body,
            current_direction: direction,
            field_width,
            field_height,
            prev_tail: None,
            direction_locked: false,
        }
    }

    pub fn default(index: usize, field_height: i8, field_width: i8) -> Self {
        match index {
            0 => {
                let mut body: LinkedList<(i8, i8)> = LinkedList::new();
                body.push_front((1, 1));
                body.push_front((2, 1));
                body.push_front((3, 1));
                Snake::new(body, Direction::Right, field_height, field_width)
            }
            1 => {
                let mut body: LinkedList<(i8, i8)> = LinkedList::new();
                body.push_front((field_width - 2, field_height - 2));
                body.push_front((field_width - 3, field_height - 2));
                body.push_front((field_width - 4, field_height - 2));
                Snake::new(body, Direction::Left, field_height, field_width)
            }
            2 => {
                let mut body: LinkedList<(i8, i8)> = LinkedList::new();
                body.push_front((field_width - 2, 1));
                body.push_front((field_width - 2, 2));
                body.push_front((field_width - 2, 3));
                Snake::new(body, Direction::Down, field_height, field_width)
            }
            3 => {
                let mut body: LinkedList<(i8, i8)> = LinkedList::new();
                body.push_front((1, field_height - 2));
                body.push_front((1, field_height - 3));
                body.push_front((1, field_height - 4));
                Snake::new(body, Direction::Up, field_height, field_width)
            }
            _ => panic!("too many snakes"),
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        match (self.current_direction, new_direction, self.direction_locked) {
            (_, _, true) => (),
            (Direction::Left, Direction::Right, _) => (),
            (Direction::Right, Direction::Left, _) => (),
            (Direction::Up, Direction::Down, _) => (),
            (Direction::Down, Direction::Up, _) => (),
            (_, v, _) => self.current_direction = v,
        }
        self.direction_locked = true;
    }

    pub fn peek(&self) -> (i8, i8) {
        let (x, y) = self.head();
        match self.current_direction {
            Direction::Left => ((x - 1 + self.field_width) % self.field_width, *y),
            Direction::Right => ((x + 1) % self.field_width, *y),
            Direction::Up => (*x, (y - 1 + self.field_height) % self.field_height),
            Direction::Down => (*x, (y + 1) % self.field_height),
        }
    }

    pub fn head(&self) -> &(i8, i8) {
        self.body.front().unwrap()
    }

    pub fn check_collisions(&self, other: &Snake) -> bool {
        other
            .body
            .iter()
            .any(|cell| cell.collides_with(self.head()))
    }

    pub fn check_collisions_with_self(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|cell| cell.collides_with(self.head()))
    }

    pub fn move_forward(&mut self, is_apple_ahead: bool) {
        self.direction_locked = false;
        self.body.push_front(self.peek());

        if !is_apple_ahead {
            self.prev_tail = self.body.pop_back();
        }
    }
}
