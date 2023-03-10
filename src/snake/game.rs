use super::directions::Direction;
use super::point::Point;
use super::snake::Snake;
use rand::{thread_rng, Rng};

pub const SIDE: u16 = 10;
const SNAKE_START_LEN: u16 = 3;

#[derive(Debug)]
pub struct Game {
    pub width: u16,
    pub height: u16,
    pub food: Option<Point>,
    placed_food: bool,
    pub snake: Snake,
    pub score: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            width: SIDE,
            height: SIDE,
            food: None,
            placed_food: false,
            snake: Snake::new(
                Point::new(SIDE / 2, SIDE / 2),
                SNAKE_START_LEN,
                Direction::rand(),
            ),
            score: 0,
        }
    }

    pub fn run(&mut self, directions: Vec<Direction>) {
        let mut slither_towards = self.snake.direction;

        if !directions.is_empty() {
            let opposite_to_current_direction = Direction::opposite(&self.snake.direction);

            let filtered_directions: Vec<Direction> =
                Direction::reject_opposite(directions, opposite_to_current_direction);

            if !filtered_directions.is_empty() {
                slither_towards = Direction::rand_from_collection(filtered_directions);
            }
        }

        self.snake.set_direction(slither_towards);

        self.snake.slither();

        if self.has_bitten_itself() {
            self.restart();
        } else if let Some(food) = self.food {
            if self.snake.get_head_point() == food {
                self.snake.grow();
                self.place_food();
                self.score += 1;
            }
        }

        if !self.placed_food {
            self.place_food();
            self.placed_food = true;
        }
    }

    pub fn place_food(&mut self) {
        loop {
            let random_x = thread_rng().gen_range(0..self.width);
            let random_y = thread_rng().gen_range(0..self.height);

            let point = Point::new(random_x, random_y);

            if !self.snake.contains_point(&point) {
                self.food = Some(point);
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn has_collided_with_wall(&self) -> bool {
        let head_point = self.snake.get_head_point();

        match self.snake.get_direction() {
            Direction::Up => head_point.y == 0,
            Direction::Right => head_point.x == self.width - 1,
            Direction::Down => head_point.y == self.height - 1,
            Direction::Left => head_point.x == 0,
        }
    }

    fn has_bitten_itself(&self) -> bool {
        let next_head_point = self
            .snake
            .get_head_point()
            .transform(self.snake.get_direction(), 1);
        let mut next_body_points = self.snake.get_body_points();
        next_body_points.remove(next_body_points.len() - 1);
        next_body_points.remove(0);

        next_body_points.contains(&next_head_point)
    }

    fn restart(&mut self) {
        self.snake = self.build_snake();
        self.placed_food = false;
        self.food = None;
        self.score = 0;
    }

    fn build_snake(&self) -> Snake {
        Snake::new(
            Point::new(SIDE / 2, SIDE / 2),
            SNAKE_START_LEN,
            Direction::rand(),
        )
    }
}
