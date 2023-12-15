use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    sys::SDL_Delay,
    AudioSubsystem,
};

#[repr(align(2))]
pub struct Sounds {
    system: AudioSubsystem,
    boom: Vec<u8>,
    food: Vec<u8>,
    start: Vec<u8>,
    wall: Vec<u8>,
    queue: Option<AudioQueue<i16>>,
}

impl Sounds {
    pub fn new(system: AudioSubsystem) -> Self {
        let boom = Vec::from(*include_bytes!("sounds/boom.wav"));
        let food = Vec::from(*include_bytes!("sounds/food.wav"));
        let start = Vec::from(*include_bytes!("sounds/start.wav"));
        let wall = Vec::from(*include_bytes!("sounds/wall.wav"));
        Self {
            boom,
            food,
            system,
            start,
            wall,
            queue: None,
        }
    }

    pub fn play(&mut self, what: &str) {
        let spec = AudioSpecDesired {
            freq: None,
            channels: Some(1u8),
            samples: None,
        };

        if self.queue.is_none() {
            self.queue = match self.system.open_queue::<i16, Option<&str>>(None, &spec) {
                Ok(q) => Some(q),
                _ => panic!("Failed to open audioqueue"),
            };
        }

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
