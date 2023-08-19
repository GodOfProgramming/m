use anyhow::Result;
use bevy::{prelude::*, window::WindowMode as BevyWindowMode};
use serde::{Deserialize, Serialize};
use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  fs,
  path::Path,
};
use strum_macros::EnumIter;

pub mod prelude {
  pub use super::{Settings, WindowMode};
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
  pub window: WindowSettings,
}

impl Settings {
  pub fn save(&self, file: &Path) -> Result<()> {
    let data = toml::to_string(self)?;
    fs::write(file, data)?;
    Ok(())
  }

  pub fn load(file: &Path) -> Result<Self> {
    let data = fs::read_to_string(file)?;
    Ok(toml::from_str(&data)?)
  }

  pub fn load_or_default(file: &Path) -> Self {
    Self::load(file).unwrap_or_else(|err| {
      warn!("Settings failed to load, using defaults. Error: {}", err);
      Self::default()
    })
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      window: WindowSettings {
        height: 720,
        width: 1280,
        mode: WindowMode::default(),
      },
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct WindowSettings {
  pub height: u32,
  pub width: u32,
  pub mode: WindowMode,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum WindowMode {
  #[default]
  Windowed,
  Fullscreen,
  Borderless,
}

impl WindowMode {
  pub fn to_bevy(self) -> BevyWindowMode {
    self.into()
  }
}

impl Display for WindowMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      WindowMode::Windowed => write!(f, "Windowed"),
      WindowMode::Fullscreen => write!(f, "Fullscreen"),
      WindowMode::Borderless => write!(f, "Borderless"),
    }
  }
}

impl From<BevyWindowMode> for WindowMode {
  fn from(value: BevyWindowMode) -> Self {
    match value {
      BevyWindowMode::Windowed => Self::Windowed,
      BevyWindowMode::BorderlessFullscreen => Self::Borderless,
      BevyWindowMode::SizedFullscreen => Self::Fullscreen,
      BevyWindowMode::Fullscreen => Self::Fullscreen,
    }
  }
}

impl Into<BevyWindowMode> for WindowMode {
  fn into(self) -> BevyWindowMode {
    match self {
      Self::Windowed => BevyWindowMode::Windowed,
      Self::Fullscreen => BevyWindowMode::Fullscreen,
      Self::Borderless => BevyWindowMode::BorderlessFullscreen,
    }
  }
}
