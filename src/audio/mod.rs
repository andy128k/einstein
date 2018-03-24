use std::cell::Cell;
use std::ops::Drop;
use failure::err_msg;
use sdl2::rwops::RWops;
use sdl2::mixer;
use sdl2::mixer::LoaderRWops;
use error::*;

pub struct Audio {
    channel: mixer::Channel,
    applause: mixer::Chunk,
    click: mixer::Chunk,
    glass: mixer::Chunk,
    laser: mixer::Chunk,
    whizz: mixer::Chunk,
    volume: Cell<f32>,
}

fn load_wav(bytes: &[u8]) -> Result<mixer::Chunk> {
    RWops::from_bytes(bytes).map_err(err_msg)?
        .load_wav().map_err(err_msg)
}

impl Audio {
    pub fn new() -> Result<Self> {
        mixer::open_audio(22050, mixer::AUDIO_S16, 2, 1024).map_err(err_msg)?;
        Ok(Audio {
            channel: mixer::Channel::all(),
            applause: load_wav(include_bytes!("./applause.wav"))?,
            click: load_wav(include_bytes!("./click.wav"))?,
            glass: load_wav(include_bytes!("./glass.wav"))?,
            laser: load_wav(include_bytes!("./laser.wav"))?,
            whizz: load_wav(include_bytes!("./whizz.wav"))?,
            volume: Cell::new(0f32),
        })
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume.set(volume);
    }

    fn play(&self, chunk: &mixer::Chunk) -> Result<()> {
        self.channel.play(chunk, 0).map_err(err_msg)?;
        Ok(())
    }

    pub fn play_applause(&self) -> Result<()> { self.play(&self.applause) }
    pub fn play_click(&self) -> Result<()> { self.play(&self.click) }
    pub fn play_glass(&self) -> Result<()> { self.play(&self.glass) }
    pub fn play_laser(&self) -> Result<()> { self.play(&self.laser) }
    pub fn play_whizz(&self) -> Result<()> { self.play(&self.whizz) }
}

impl Drop for Audio {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}
