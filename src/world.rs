use rand::Rng;

pub const FIELD_SIZE: u32 = 30;
pub const FOOD_LIFETIME: u32 = 60;

pub enum StepError {
    OutOfField,
    SelfHit,
}

pub enum StepOk {
    Nothing,
    AteFood,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const SNAKE_INIT_X: u32 = FIELD_SIZE / 2;
const SNAKE_INIT_Y: u32 = FIELD_SIZE / 2;
const SNAKE_INIT_DIR: Direction = Direction::Up;

#[derive(PartialEq)]
pub enum Thing {
    Food,
}

pub struct World {
    pub snake: Vec<(u32, u32)>,
    snake_dir: Direction,
    // what is it, index of the corresponding picture, coordinates, possible lifetime
    pub things: Vec<(Thing, usize, u32, u32, Option<u32>)>,
    pub score: u32,
    grow: i32, // grow for this amount of turns; 0 means do not grow
}

impl World {
    pub fn init() -> Self {
        World {
            snake: vec![
                (SNAKE_INIT_X, SNAKE_INIT_Y),
                (SNAKE_INIT_X, SNAKE_INIT_Y + 1),
                (SNAKE_INIT_X, SNAKE_INIT_Y + 2),
            ],
            snake_dir: SNAKE_INIT_DIR,
            things: vec![],
            grow: 0,
            score: 0,
        }
    }
    fn empty_spot(&self) -> (u32, u32) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..FIELD_SIZE) + 1;
            let y = rng.gen_range(0..FIELD_SIZE) + 1;
            if self.snake.contains(&(x, y)) {
                continue;
            }
            if self.things.iter().any(|t| t.2 == x && t.3 == y) {
                continue;
            }
            return (x, y);
        }
    }
    pub fn step(&mut self) -> Result<StepOk, StepError> {
        let (mut next_x, mut next_y) = self.snake[0];
        let mut step_ok = StepOk::Nothing;

        match self.snake_dir {
            Direction::Up => {
                if next_y > 1 {
                    next_y -= 1;
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Down => {
                if next_y < FIELD_SIZE {
                    next_y += 1
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Left => {
                if next_x > 1 {
                    next_x -= 1;
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Right => {
                if next_x < FIELD_SIZE {
                    next_x += 1
                } else {
                    return Err(StepError::OutOfField);
                }
            }
        };
        // Check if we ate something and update lifetimes
        let mut extra: Vec<usize> = vec![];
        for (idx, thing) in self.things.iter_mut().enumerate() {
            match thing.4 {
                Some(0) =>
                // the thing is expired
                {
                    extra.push(idx);
                }
                Some(n) => {
                    thing.4 = Some(n - 1);
                }
                _ => {}
            }

            if thing.2 != next_x || thing.3 != next_y {
                continue;
            }
            extra.push(idx);
            match thing.0 {
                Thing::Food => {
                    self.score += 1;
                    self.grow += 3;
                    step_ok = StepOk::AteFood;
                }
            };
        }

        for (offset, r) in extra.iter().enumerate() {
            self.things.swap_remove(r - offset);
        }
        // Maybe shrink snake
        if self.grow == 0 {
            self.snake.pop();
        } else {
            assert!(
                self.grow > 0,
                "Programming error: the number of turns snake grows cannot be negative"
            );
            self.grow -= 1;
        }
        // Check if we hit snake
        if self.snake.contains(&(next_x, next_y)) {
            return Err(StepError::SelfHit);
        }
        self.snake.insert(0, (next_x, next_y));
        Ok(step_ok)
    }

    pub fn turn_left(&mut self) {
        self.snake_dir = match self.snake_dir {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        };
    }

    pub fn turn_right(&mut self) {
        self.snake_dir = match self.snake_dir {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
        };
    }

    pub fn add_thing(&mut self) {
        let (x, y) = self.empty_spot();
        let mut rng = rand::thread_rng();
        self.things
            .push((Thing::Food, rng.gen_range(0..3), x, y, Some(FOOD_LIFETIME)));
    }
}
