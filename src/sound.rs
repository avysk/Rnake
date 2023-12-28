use sdl2::{audio, audio::AudioSpecDesired, sys::SDL_Delay, AudioSubsystem};

struct NoSound;

macro_rules! with_sounds {
    ($($sound:ident)*) => {
        pub trait Player {
            $(fn $sound(&self) {})*
        }

        impl Player for NoSound {
            $(fn $sound(&self) {
                // no sound, do nothing
            })*
        }

        #[repr(align(2))]
        pub struct Sounds {
            queue: audio::AudioQueue<i16>,
            $($sound: Vec<u8>),*
        }

        impl Player for Sounds {
            $(fn $sound(&self) {
                self.queue.queue_audio(bytemuck::cast_slice::<u8, i16>(self.$sound.as_slice()))
                    .expect("Should be able to queue audio");
                self.queue.resume();
            })*
        }

        impl Sounds {
            pub fn create(maybe_system: Result<AudioSubsystem, String>) -> Box<dyn Player> {
                if let Ok(system) = maybe_system {
                let spec = AudioSpecDesired {
                    freq: None,
                    channels: Some(1u8),
                    samples: None,
                };
                let queue = system
                    .open_queue::<i16, Option<&str>>(None, &spec)
                    .expect("Should be able to open AudioQueue");
                $(let $sound = Vec::from(*include_bytes!(concat!("sounds/", stringify!($sound), ".wav")));)*
                Box::new(Sounds {
                    queue,
                    $($sound),*
                }) } else { // no audiosystem
                    Box::new(NoSound {})
                }
            }
        }
    }
}

with_sounds!(boom food obstacle start wall);

impl Drop for Sounds {
    fn drop(&mut self) {
        while self.queue.size() > 0 {
            unsafe {
                SDL_Delay(20);
            }
        }
    }
}
