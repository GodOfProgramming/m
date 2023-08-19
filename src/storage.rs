mod saves;
mod settings;

use anyhow::Result;
use bevy::prelude::*;
use std::path::PathBuf;

pub use settings::prelude::*;

#[derive(Resource)]
pub struct SystemInformation {
  pub settings_path: PathBuf,
  pub settings: Settings,
  pub focused_entity: Option<Focus>,
}

pub struct Focus {
  pub handle: Entity,
  pub on_chars_received: fn(Entity, char, &mut Query<&mut Text>),
}

impl SystemInformation {
  pub fn new(settings_path: PathBuf, settings: Settings) -> Self {
    Self {
      settings_path,
      settings,
      focused_entity: None,
    }
  }

  pub fn save_settings(&self) -> Result<()> {
    self.settings.save(&self.settings_path)
  }
}
