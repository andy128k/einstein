use crate::error::*;
use crate::ui::component::game::GamePrivate;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn read_file(filename: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_file(filename: &Path, buf: &[u8]) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(buf)?;
    Ok(())
}

fn app_dir() -> Result<PathBuf> {
    let home = home_dir().ok_or_else(|| format_err!("Home directory is not detected."))?;
    let dir = home.join(".einstein");
    Ok(dir)
}

fn storage_path() -> Result<PathBuf> {
    Ok(app_dir()?.join("einstein.json"))
}

pub const MAX_SLOTS: usize = 10;
pub const MAX_SCORES: usize = 10;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Score {
    pub name: String,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Scores(pub Vec<Score>);

impl Scores {
    pub fn init(&mut self) {
        self.0.sort_by_key(|score| score.score);
        self.0.truncate(MAX_SCORES);
    }

    pub fn is_deserving(&self, challenger: u32) -> bool {
        self.0.len() < MAX_SCORES
            || self
                .0
                .last()
                .map(|last| challenger < last.score)
                .unwrap_or(false)
    }

    pub fn add_score_entry(&mut self, entry: Score) -> Option<usize> {
        if self.0.is_empty() {
            self.0.push(entry);
            return Some(0);
        }
        let index = self.0.iter().position(|ref e| e.score > entry.score);
        match index {
            Some(index) if index < MAX_SCORES => {
                self.0.insert(index, entry);
                self.0.truncate(MAX_SCORES);
                Some(index)
            }
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedGame {
    pub name: String,
    pub game: GamePrivate,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Storage {
    pub fullscreen: bool,
    pub volume: u32,
    pub last_name: Option<String>,
    pub scores: Scores,
    pub saved_games: [Option<SavedGame>; MAX_SLOTS],
}

impl Storage {
    pub fn load_from_file(filename: &Path) -> Result<Self> {
        let buf = read_file(filename)?;
        let mut storage: Storage = serde_json::from_slice(&buf)?;
        storage.scores.init();
        Ok(storage)
    }

    pub fn load() -> Result<Self> {
        Self::load_from_file(&storage_path()?)
    }

    pub fn save_to_file(&self, filename: &Path) -> Result<()> {
        let dump = serde_json::to_vec_pretty(self)?;
        write_file(&filename, &dump)?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        create_dir_all(app_dir()?)?;
        self.save_to_file(&storage_path()?)
    }
}
