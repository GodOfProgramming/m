use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{game::GameState, storage::SystemInformation};

#[derive(Resource)]
pub struct CharacterSelectionMenu {
  characters: Vec<String>,
}

pub fn on_enter(mut commands: Commands, sys_info: Res<SystemInformation>) {
  let mut char_names = Vec::new();
  if let Ok(dir) = sys_info.game_saves_path.read_dir() {
    for file in dir {
      if let Ok(file) = file {
        if let Ok(kind) = file.file_type() {
          if kind.is_file() {
            if let Some(ext) = file.path().extension() {
              if ext == "ms" {
                if let Some(name) = file.file_name().to_str() {
                  char_names.push(name.to_string());
                }
              }
            }
          }
        }
      }
    }
  }

  commands.insert_resource(CharacterSelectionMenu {
    characters: char_names,
  })
}

pub fn on_update(
  mut next_state: ResMut<NextState<GameState>>,
  mut contexts: EguiContexts,
  menu: Res<CharacterSelectionMenu>,
) {
  egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    if ui.button("New Character").clicked() {
      next_state.set(GameState::CharacterCreate);
    }

    if !menu.characters.is_empty() {
      ui.separator();

      ui.label("Select Character");

      for char_name in &menu.characters {
        if ui.button(char_name).clicked() {
          next_state.set(GameState::StartGame);
        }
      }
    }
  });
}

pub fn on_exit(mut commands: Commands) {
  commands.remove_resource::<CharacterSelectionMenu>();
}
