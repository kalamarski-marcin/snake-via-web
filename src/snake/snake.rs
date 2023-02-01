use super::directions::Direction;
use super::point::Point;

#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
    pub digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();

        Self {
            body,
            direction,
            digesting: false,
        }
    }

    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) {
        self.body
            .insert(0, self.body.first().unwrap().transform(self.direction, 1));
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        } else {
            self.digesting = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_snake() {
        let head: Point = Point::new(5, 5);
        let snake = Snake::new(head, 3, Direction::Left);

        assert_eq!(head, snake.get_head_point());

        let second_point = Point::new(6, 5);
        let third_point = Point::new(7, 5);

        let after_snake = Point::new(8, 5);
        let before_snake = Point::new(4, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        assert!(!snake.contains_point(&before_snake));
        assert!(!snake.contains_point(&after_snake));
    }

    #[test]
    fn slither_down() {
        let head: Point = Point::new(5, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Down);

        snake.slither();

        let new_head: Point = Point::new(5, 6);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 5);
        let third_point = Point::new(5, 4);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_down_through_wall() {
        let head: Point = Point::new(5, 9);
        let mut snake: Snake = Snake::new(head, 3, Direction::Down);

        snake.slither();

        let new_head: Point = Point::new(5, 0);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 9);
        let third_point = Point::new(5, 8);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(5, 1);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 0);
        let third_point = Point::new(5, 9);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(5, 2);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 1);
        let third_point = Point::new(5, 0);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_up() {
        let head: Point = Point::new(5, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Up);

        snake.slither();

        let new_head: Point = Point::new(5, 4);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 5);
        let third_point = Point::new(5, 6);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_up_through_wall() {
        let head: Point = Point::new(5, 0);
        let mut snake: Snake = Snake::new(head, 3, Direction::Up);

        snake.slither();

        let new_head: Point = Point::new(5, 9);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 0);
        let third_point = Point::new(5, 1);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(5, 8);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 9);
        let third_point = Point::new(5, 0);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(5, 7);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 8);
        let third_point = Point::new(5, 9);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_right() {
        let head: Point = Point::new(5, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Right);

        snake.slither();

        let new_head: Point = Point::new(6, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 5);
        let third_point = Point::new(4, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_right_through_wall() {
        let head: Point = Point::new(9, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Right);

        snake.slither();

        let new_head: Point = Point::new(0, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(9, 5);
        let third_point = Point::new(8, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(1, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(0, 5);
        let third_point = Point::new(9, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(2, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(1, 5);
        let third_point = Point::new(0, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_left() {
        let head: Point = Point::new(5, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Left);

        snake.slither();

        let new_head: Point = Point::new(4, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(5, 5);
        let third_point = Point::new(6, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }

    #[test]
    fn slither_left_through_wall() {
        let head: Point = Point::new(0, 5);
        let mut snake: Snake = Snake::new(head, 3, Direction::Left);

        snake.slither();

        let new_head: Point = Point::new(9, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(0, 5);
        let third_point = Point::new(1, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(8, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(9, 5);
        let third_point = Point::new(0, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));

        snake.slither();

        let new_head: Point = Point::new(7, 5);
        assert_eq!(new_head, snake.get_head_point());

        let second_point = Point::new(8, 5);
        let third_point = Point::new(9, 5);

        assert!(snake.contains_point(&second_point));
        assert!(snake.contains_point(&third_point));
    }
}
