use rand::Rng;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Front => Direction::Back,
            Direction::Back => Direction::Front,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
            Direction::Front => write!(f, "F"),
            Direction::Back => write!(f, "B"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Turn {
    Clockwise,
    CounterClockwise,
    Double,
}

impl Turn {
    pub fn opposite(&self) -> Self {
        match self {
            Turn::Clockwise => Turn::CounterClockwise,
            Turn::CounterClockwise => Turn::Clockwise,
            Turn::Double => Turn::Double,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::Clockwise => write!(f, ""),
            Turn::CounterClockwise => write!(f, "'"),
            Turn::Double => write!(f, "2"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Move {
    pub direction: Direction,
    pub turn: Turn,
}

impl Move {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let direction = match rng.gen_range(0..6) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::Front,
            5 => Direction::Back,
            _ => panic!("Random number generator failed"),
        };
        let turn = match rng.gen_range(0..3) {
            0 => Turn::Clockwise,
            1 => Turn::CounterClockwise,
            2 => Turn::Double,
            _ => panic!("Random number generator failed"),
        };
        Move { direction, turn }
    }
}
