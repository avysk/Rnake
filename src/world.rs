use rand::Rng;

pub const FIELD_SIZE: usize = 30;

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

const SNAKE_INIT_X: usize = FIELD_SIZE / 2;
const SNAKE_INIT_Y: usize = FIELD_SIZE / 2;
const SNAKE_INIT_DIR: Direction = Direction::Up;

#[derive(PartialEq)]
pub enum Thing {
    Food,
}

pub struct World {
    pub snake: Vec<(usize, usize)>,
    snake_dir: Direction,
    pub things: Vec<(Thing, usize, usize)>,
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
        }
    }
    pub fn step(&mut self) -> Result<StepOk, StepError> {
        let (mut next_x, mut next_y) = self.snake[0];
        let mut step_ok = StepOk::Nothing;

        match self.snake_dir {
            Direction::Up => {
                if next_y > 0 {
                    next_y -= 1;
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Down => {
                if next_y < FIELD_SIZE - 1 {
                    next_y += 1
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Left => {
                if next_x > 0 {
                    next_x -= 1;
                } else {
                    return Err(StepError::OutOfField);
                }
            }
            Direction::Right => {
                if next_x < FIELD_SIZE - 1 {
                    next_x += 1
                } else {
                    return Err(StepError::OutOfField);
                }
            }
        };
        // Check if we ate something
        let idx = self
            .things
            .iter()
            .position(|x| x.1 == next_x && x.2 == next_y);
        if let Some(pos) = idx {
            match self.things[pos].0 {
                Thing::Food => {
                    self.grow += 3;
                    step_ok = StepOk::AteFood;
                }
            };
            self.things.swap_remove(pos);
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
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..FIELD_SIZE);
            let y = rng.gen_range(0..FIELD_SIZE);
            if self.snake.contains(&(x, y)) {
                continue;
            }
            if self.things.iter().any(|t| t.1 == x && t.2 == y) {
                continue;
            }
            self.things.push((Thing::Food, x, y));
            break;
        }
    }
}
