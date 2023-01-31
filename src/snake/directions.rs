use rand::{thread_rng, Rng};

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn from_str(direction: &str) -> Option<Self> {
        match direction {
            "down" => Some(Self::Down),
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "up" => Some(Self::Up),
            _ => None,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn reject_opposite(directions: Vec<Direction>, opposite: Direction) -> Vec<Direction> {
        let filtered: Vec<Direction> = directions.into_iter().filter(|m| m != &opposite).collect();

        filtered
    }

    pub fn rand_from_collection(directions: Vec<Direction>) -> Direction {
        directions[thread_rng().gen_range(0..directions.len())]
    }

    pub fn rand() -> Direction {
        match thread_rng().gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        }
    }
}
