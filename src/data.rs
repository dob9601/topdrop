use std::{collections::HashMap, error::Error, fs::OpenOptions, path::PathBuf};

use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref BASE_PATH: PathBuf = ProjectDirs::from("com", "dob9601", "dropper")
        .unwrap()
        .cache_dir()
        .to_path_buf();
    static ref STATE_PATH: PathBuf = BASE_PATH.join("state.yaml");
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(flatten)]
    pub data: HashMap<String, ApplicationState>,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn open() -> Result<Self, Box<dyn Error>> {
        std::fs::create_dir_all(&*BASE_PATH)?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&*STATE_PATH)?;

        Ok(serde_yaml::from_reader(file).unwrap_or_else(|_err| Self::new()))
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&*BASE_PATH)?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&*STATE_PATH)?;

        Ok(serde_yaml::to_writer(file, &self)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationState {
    pub id: u32,
    pub visible: bool,
}

impl ApplicationState {
    pub fn new(id: u32, visible: bool) -> Self {
        Self { id, visible }
    }
}
