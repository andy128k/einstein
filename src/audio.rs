use std::ops::Drop;
use sdl2::mixer;

pub struct Audio {
    channel: mixer::Channel,
}

impl Audio {
    pub fn new() -> Result<Self, String> {
        mixer::open_audio(22050, mixer::AUDIO_S16, 2, 1024)?;
        Ok(Audio {
            channel: mixer::Channel::all(),
        })
    }

    pub fn set_volume(&self, volume: u32) {
        self.channel.set_volume((volume as i32) * mixer::MAX_VOLUME / 100);
    }

    pub fn play(&self, chunk: &mixer::Chunk) -> Result<(), String> {
        self.channel.play(chunk, 0)?;
        Ok(())
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}
