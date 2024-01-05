use sdl2::mixer::{Channel, Chunk, Music, DEFAULT_CHANNELS, DEFAULT_FORMAT, DEFAULT_FREQUENCY};
use std::collections::HashMap;

struct NoSound;

pub struct Sounds {
    chunks: HashMap<String, Chunk>,
    music: Music<'static>,
}

macro_rules! with_sounds {
    ($($sound:ident)*) => {
        pub trait Player {
            fn play_music(&self) {}
            $(fn $sound(&self) {})*
        }

        impl Player for NoSound {
            fn play_music(&self) {
                // no sound, do nothing
            }
            $(fn $sound(&self) {
                // no sound, do nothing
            })*
        }

        impl Player for Sounds {
            fn play_music(&self) {
                self.music.play(-1).expect("Sound is supported, should be able to play music");
            }
            $(fn $sound(&self) {
                Channel(-1).play(&self.chunks[stringify!($sound)], 0).expect("Should be able to play sound");
            })*
        }

        impl Sounds {
            pub fn create() -> Box<dyn Player> {
                let mix = sdl2::mixer::init(sdl2::mixer::InitFlag::all());
                if mix.is_err() {
                    return Box::new(NoSound {});
                }
                let audio = sdl2::mixer::open_audio(
                    DEFAULT_FREQUENCY,
                    DEFAULT_FORMAT,
                    DEFAULT_CHANNELS,
                    512
                );
                if audio.is_err() {
                    panic!("SDL2 mixer init succeeded, should be able to open audio");
                }
                sdl2::mixer::allocate_channels(8);
                let music = sdl2::mixer::Music::from_static_bytes(include_bytes!("resources/sounds/kim-lightyear-just-a-dream-wake-up.wav")).expect("Should be able to create music");
                let mut chunks = HashMap::new();
                $(
                let $sound = sdl2::mixer::Chunk::from_raw_buffer(Box::new(*include_bytes!(concat!("resources/sounds/", stringify!($sound), ".wav")))).expect("Should be able to create chunk");
                chunks.insert(stringify!($sound).to_string(), $sound);
                    )*
                Box::new(Sounds {
                    music,
                    chunks
                })
            }
        }
    }
}

with_sounds!(boom food mystery obstacle start wall);
