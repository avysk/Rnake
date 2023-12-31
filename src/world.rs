use rand::{distributions::Uniform, Rng};

pub const FIELD_SIZE: u32 = 30;
pub const FOOD_LIFETIME: u32 = 60;
pub const OBSTACLE_LIFETIME: u32 = 60;
pub const OBSTACLE_P: f32 = 0.015;
pub const MYSTERY_P: f32 = 0.0025;
pub const MYSTERY_LIFETIME: u32 = 120;
pub const MYSTERY_SCORE: u32 = 5;
pub const MYSTERY_GROW_SNAKE: u32 = 15;
pub const LEAN_P: f32 = 0.5;
pub const LEAN_AFTER_FOOD: u32 = 5;
pub const LEAN_LIFETIME: u32 = 60;
pub const FAT_P: f32 = 0.1;
pub const FAT_GROW_SNAKE: u32 = 6;

pub enum StepError {
    Obstacle,
    OutOfField,
    SelfHit,
}

pub enum StepOk {
    Nothing,
    AteFood,
    AteMystery,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Thing {
    Fat,
    Food,
    Lean,
    Mystery,
    Obstacle,
}

#[derive(Clone, Debug)]
pub struct ThingInField {
    pub what: Thing,
    pub picture_index: usize,
    // coordinates are from 1 to FIELD_SIZE
    pub x: u32,
    pub y: u32,
    lifetime: Option<u32>,
}

pub struct World {
    pub snake: Vec<(u32, u32)>,
    snake_dir: Direction,
    // what is it, index of the corresponding picture, coordinates, possible lifetime
    pub things: Vec<ThingInField>,
    pub score: u32,
    grow: u32, // grow for this amount of turns; 0 means do not grow
    eaten_food: u32,
}

impl World {
    pub fn init() -> Self {
        let mut w = World {
            snake: vec![
                (SNAKE_INIT_X, SNAKE_INIT_Y),
                (SNAKE_INIT_X, SNAKE_INIT_Y + 1),
                (SNAKE_INIT_X, SNAKE_INIT_Y + 2),
            ],
            snake_dir: SNAKE_INIT_DIR,
            things: vec![],
            grow: 0,
            score: 0,
            eaten_food: 0,
        };
        w.add_food();
        w
    }
    fn empty_spot(&self) -> (u32, u32) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..FIELD_SIZE) + 1;
            let y = rng.gen_range(0..FIELD_SIZE) + 1;
            if self.snake.contains(&(x, y)) {
                continue;
            }
            // prevent things appearing next to snake
            let hx = self.snake[0].0;
            let hy = self.snake[0].1;
            if x < hx + 3 && y < hy + 3 && x > hx.saturating_sub(3) && y > hy.saturating_sub(3) {
                continue;
            }
            if self.things.iter().any(|t| t.x == x && t.y == y) {
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

        // Start by moving a snake so when we create new things snake position is updated

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

        // Now go through things, check if we hit something, update lifetimes
        let mut deleted = 0;
        for (idx, thing) in self.things.clone().iter_mut().enumerate() {
            match thing.lifetime {
                Some(0) =>
                // the thing is expired
                {
                    if thing.what == Thing::Food || thing.what == Thing::Lean {
                        self.add_food();
                    }
                    self.things.remove(idx - deleted);
                    deleted += 1;
                    continue; // if the thing is expired, do not check if we hit it
                }
                _ if thing.x == next_x && thing.y == next_y => {
                    // we hit it
                    self.things.remove(idx - deleted);
                    deleted += 1;
                    match thing.what {
                        Thing::Obstacle => {
                            return Err(StepError::Obstacle);
                        }
                        Thing::Food => {
                            self.eaten_food += 1;
                            self.score += 1;
                            self.grow += 3;
                            step_ok = StepOk::AteFood;
                            self.add_food();
                        }
                        Thing::Fat => {
                            self.eaten_food += 1;
                            self.score += 1;
                            self.grow += FAT_GROW_SNAKE;
                            step_ok = StepOk::AteFood;
                            self.add_food();
                        }
                        Thing::Lean => {
                            self.eaten_food = 0;
                            self.score += 1;
                            step_ok = StepOk::AteFood;
                            self.add_food();
                        }
                        Thing::Mystery => {
                            let mut rng = rand::thread_rng();
                            if rng.sample(Uniform::new(0.0, 1.0)) < 0.5 {
                                self.score += MYSTERY_SCORE;
                                self.eaten_food += 1;
                            } else {
                                self.grow += MYSTERY_GROW_SNAKE;
                            }
                            step_ok = StepOk::AteMystery;
                        }
                    }
                }
                Some(n) => {
                    // We did not hit it
                    self.things.remove(idx - deleted);
                    deleted += 1;
                    self.things.push(ThingInField {
                        lifetime: Some(n - 1),
                        ..thing.clone()
                    });
                    continue;
                    // we hit it
                }
                _ => {}
            }
        }

        self.maybe_add_obstacle();
        self.maybe_add_mystery();

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

    fn add_food(&mut self) {
        let (x, y) = self.empty_spot();
        let mut rng = rand::thread_rng();
        if self.eaten_food >= LEAN_AFTER_FOOD && rng.sample(Uniform::new(0.0, 1.0)) < LEAN_P {
            self.things.push(ThingInField {
                what: Thing::Lean,
                picture_index: rng.gen_range(0..3),
                x,
                y,
                lifetime: Some(LEAN_LIFETIME),
            });
            return;
        }
        if rng.sample(Uniform::new(0.0, 1.0)) < FAT_P {
            self.things.push(ThingInField {
                what: Thing::Fat,
                picture_index: rng.gen_range(0..3),
                x,
                y,
                lifetime: None,
            });
            return;
        }

        let lifetime = if x == 1 || y == 1 || x == FIELD_SIZE || y == FIELD_SIZE {
            None
        } else {
            Some(FOOD_LIFETIME)
        };
        let mut rng = rand::thread_rng();
        self.things.push(ThingInField {
            what: Thing::Food,
            picture_index: rng.gen_range(0..3),
            x,
            y,
            lifetime,
        });
    }

    fn maybe_add_obstacle(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.sample(Uniform::new(0.0, 1.0)) > OBSTACLE_P {
            return;
        }
        let (x, y) = self.empty_spot();
        self.things.push(ThingInField {
            what: Thing::Obstacle,
            picture_index: rng.gen_range(0..3),
            x,
            y,
            lifetime: Some(OBSTACLE_LIFETIME),
        });
    }

    fn maybe_add_mystery(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.sample(Uniform::new(0.0, 1.0)) > MYSTERY_P {
            return;
        }
        let (x, y) = self.empty_spot();
        self.things.push(ThingInField {
            what: Thing::Mystery,
            picture_index: rng.gen_range(0..4),
            x,
            y,
            lifetime: Some(MYSTERY_LIFETIME),
        });
    }
}
