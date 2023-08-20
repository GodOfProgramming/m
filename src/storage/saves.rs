use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
  version: u16,
  pub name: String,
  pub attributes: Attributes,
}

impl SaveData {
  pub const LATEST: u16 = 1;
}

// different than gameplay attributes
// which contains methods for dynamic
// values
#[derive(Clone, Serialize, Deserialize)]
pub struct Attributes {
  pub vitality: u32,
  pub endurance: u32,
  pub strength: u32,
  pub dexterity: u32,
  pub agility: u32,
  pub intelligence: u32,
  pub wisdom: u32,
  pub mind: u32,
}

impl Default for Attributes {
  fn default() -> Self {
    Self {
      vitality: 1,
      endurance: 1,
      strength: 1,
      dexterity: 1,
      agility: 1,
      intelligence: 1,
      wisdom: 1,
      mind: 1,
    }
  }
}

pub struct SaveDataBuilder {
  data: SaveData,
}

impl SaveDataBuilder {
  pub fn new() -> Self {
    Self {
      data: SaveData {
        version: SaveData::LATEST,
        name: String::default(),
        attributes: Attributes::default(),
      },
    }
  }

  pub fn name(mut self, name: String) -> Self {
    self.data.name = name;
    self
  }

  pub fn attributes(mut self, attributes: Attributes) -> Self {
    self.data.attributes = attributes;
    self
  }

  pub fn build(self) -> SaveData {
    self.data
  }
}
