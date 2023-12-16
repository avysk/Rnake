use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    sys::SDL_Delay,
    AudioSubsystem,
};

pub trait Player {
    fn play(&self, what: &str);
}

struct NoSound;

impl Player for NoSound {
    fn play(&self, _what: &str) {
        // no sound, do nothing
    }
}

#[repr(align(2))]
struct Sounds {
    boom: Vec<u8>,
    food: Vec<u8>,
    start: Vec<u8>,
    wall: Vec<u8>,
    queue: Option<AudioQueue<i16>>,
}

impl Player for Sounds {
    fn play(&self, what: &str) {
        let data = match what {
            "boom" => &self.boom,
            "food" => &self.food,
            "start" => &self.start,
            "wall" => &self.wall,
            s => panic!("Programming error: unknown sound '{}'", s),
        };
        let q = self.queue.as_ref().expect("Should have Audioqueue");
        q.queue_audio(bytemuck::cast_slice::<u8, i16>(data.as_slice()))
            .expect("Should be able to queue audio");
        q.resume();
    }
}

pub fn new_player(maybe_system: Result<AudioSubsystem, String>) -> Box<dyn Player> {
    if let Ok(system) = maybe_system {
        let spec = AudioSpecDesired {
            freq: None,
            channels: Some(1u8),
            samples: None,
        };
        let queue = match system.open_queue::<i16, Option<&str>>(None, &spec) {
            Ok(q) => Some(q),
            _ => panic!("Failed to open audioqueue"),
        };
        let boom = Vec::from(*include_bytes!("sounds/boom.wav"));
        let food = Vec::from(*include_bytes!("sounds/food.wav"));
        let start = Vec::from(*include_bytes!("sounds/start.wav"));
        let wall = Vec::from(*include_bytes!("sounds/wall.wav"));
        Box::new(Sounds {
            boom,
            food,
            start,
            wall,
            queue,
        })
    } else {
        Box::new(NoSound {})
    }
}

impl Drop for Sounds {
    fn drop(&mut self) {
        if let Some(q) = self.queue.as_ref() {
            while q.size() > 0 {
                unsafe {
                    SDL_Delay(20);
                }
            }
        }
    }
}
