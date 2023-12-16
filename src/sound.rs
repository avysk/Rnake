use sdl2::{audio, audio::AudioSpecDesired, sys::SDL_Delay, AudioSubsystem};

pub trait Player {
    fn play(&self, what: &str);
}

struct NoSound;

impl Player for NoSound {
    fn play(&self, _what: &str) {
        // no sound, do nothing
    }
}

macro_rules! with_sounds {
    ($($sound:ident)*) => {
        #[repr(align(2))]
        struct Sounds {
            queue: audio::AudioQueue<i16>,
            $($sound: Vec<u8>),*
        }

        impl Player for Sounds {
            fn play(&self, what: &str) {
                let data = match what {
                    $(stringify!($sound) => &self.$sound),*,
                    s => panic!("Programming error: unknown sound '{}'", s),
                };
                self.queue.queue_audio(bytemuck::cast_slice::<u8, i16>(data.as_slice()))
                    .expect("Should be able to queue audio");
                self.queue.resume();
            }
        }

        impl Sounds {
            fn new(system: AudioSubsystem) -> Sounds {
                let spec = AudioSpecDesired {
                    freq: None,
                    channels: Some(1u8),
                    samples: None,
                };
                let queue = system
                    .open_queue::<i16, Option<&str>>(None, &spec)
                    .expect("Should be able to open AudioQueue");
                $(let $sound = Vec::from(*include_bytes!(concat!("sounds/", stringify!($sound), ".wav")));)*
                Sounds {
                    queue,
                    $($sound),*
                }
            }
        }
    }
}

with_sounds!(boom food start wall);

pub fn new_player(maybe_system: Result<AudioSubsystem, String>) -> Box<dyn Player> {
    if let Ok(system) = maybe_system {
        Box::new(Sounds::new(system))
    } else {
        Box::new(NoSound {})
    }
}

impl Drop for Sounds {
    fn drop(&mut self) {
        while self.queue.size() > 0 {
            unsafe {
                SDL_Delay(20);
            }
        }
    }
}
