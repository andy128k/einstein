use sdl2::mixer;
use std::ops::Drop;

pub trait Audio {
    fn set_volume(&self, volume: u32);
    fn play(&self, chunk: &mixer::Chunk) -> Result<(), String>;
}

pub struct SdlAudio {
    channel: mixer::Channel,
}

impl SdlAudio {
    pub fn new() -> Result<Self, String> {
        mixer::open_audio(22050, mixer::AUDIO_S16, 2, 1024)?;
        Ok(SdlAudio {
            channel: mixer::Channel::all(),
        })
    }
}

impl Audio for SdlAudio {
    fn set_volume(&self, volume: u32) {
        self.channel
            .set_volume((volume as i32) * mixer::MAX_VOLUME / 100);
    }

    fn play(&self, chunk: &mixer::Chunk) -> Result<(), String> {
        self.channel.play(chunk, 0)?;
        Ok(())
    }
}

impl Drop for SdlAudio {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}
