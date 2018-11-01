use std::cell::Cell;
use std::ops::Drop;
use sdl2::mixer;

pub struct Audio {
    channel: mixer::Channel,
    volume: Cell<f32>,
}

impl Audio {
    pub fn new() -> Result<Self, String> {
        mixer::open_audio(22050, mixer::AUDIO_S16, 2, 1024)?;
        Ok(Audio {
            channel: mixer::Channel::all(),
            volume: Cell::new(0f32),
        })
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume.set(volume);
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
