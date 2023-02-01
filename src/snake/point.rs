use super::directions::Direction;
use super::game::SIDE;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
        };

        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by < 0 {
            if value == 0 {
                SIDE - by.abs() as u16
            } else {
                value - by.abs() as u16
            }
        } else {
            if value + by as u16 == SIDE {
                0
            } else {
                value + by as u16
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewind_right() {
        let point: Point = Point::new(9, 5);
        let transformed = point.transform(Direction::Right, 1);

        assert_eq!(0, transformed.x);
        assert_eq!(5, transformed.y);
    }

    #[test]
    fn transform_right() {
        let point: Point = Point::new(5, 5);
        let transformed = point.transform(Direction::Right, 1);

        assert_eq!(6, transformed.x);
        assert_eq!(5, transformed.y);
    }

    #[test]
    fn rewind_down() {
        let point: Point = Point::new(2, 9);
        let transformed = point.transform(Direction::Down, 1);

        assert_eq!(2, transformed.x);
        assert_eq!(0, transformed.y);
    }

    #[test]
    fn transform_down() {
        let point: Point = Point::new(5, 5);
        let transformed = point.transform(Direction::Down, 1);

        assert_eq!(5, transformed.x);
        assert_eq!(6, transformed.y);
    }

    #[test]
    fn rewind_up() {
        let point: Point = Point::new(2, 0);
        let transformed = point.transform(Direction::Up, 1);

        assert_eq!(2, transformed.x);
        assert_eq!(9, transformed.y);
    }

    #[test]
    fn transform_up() {
        let point: Point = Point::new(5, 5);
        let transformed = point.transform(Direction::Up, 1);

        assert_eq!(5, transformed.x);
        assert_eq!(4, transformed.y);
    }

    #[test]
    fn rewind_left() {
        let point: Point = Point::new(0, 5);
        let transformed = point.transform(Direction::Left, 1);

        assert_eq!(9, transformed.x);
        assert_eq!(5, transformed.y);
    }

    #[test]
    fn transform_left() {
        let point: Point = Point::new(5, 5);
        let transformed = point.transform(Direction::Left, 1);

        assert_eq!(4, transformed.x);
        assert_eq!(5, transformed.y);
    }
}
