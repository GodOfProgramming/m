pub mod saves;
pub mod settings;

use anyhow::Result;
use bevy::prelude::*;
use std::path::PathBuf;

pub use settings::prelude::*;

#[derive(Resource)]
pub struct SystemInformation {
  pub game_saves_path: PathBuf,
  pub settings_path: PathBuf,
  pub settings: Settings,
}

impl SystemInformation {
  pub fn new(game_saves_path: PathBuf, settings_path: PathBuf, settings: Settings) -> Self {
    Self {
      game_saves_path,
      settings_path,
      settings,
    }
  }

  pub fn save_settings(&self) -> Result<()> {
    self.settings.save(&self.settings_path)
  }
}
