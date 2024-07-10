use std::collections::LinkedList;

pub trait Collidable<T> {
    fn collides_with(&self, other: &T) -> bool;
}

impl Collidable<(i8, i8)> for (i8, i8) {
    fn collides_with(&self, other: &(i8, i8)) -> bool {
        self == other
    }
}

impl Collidable<LinkedList<(i8, i8)>> for (i8, i8) {
    fn collides_with(&self, group: &LinkedList<(i8, i8)>) -> bool {
        group.contains(self)
    }
}

impl Collidable<(i8, i8)> for LinkedList<(i8, i8)> {
    fn collides_with(&self, other: &(i8, i8)) -> bool {
        self.contains(other)
    }
}

impl Collidable<LinkedList<(i8, i8)>> for LinkedList<(i8, i8)> {
    fn collides_with(&self, other: &LinkedList<(i8, i8)>) -> bool {
        self.iter().any(|item| other.contains(item))
    }
}
