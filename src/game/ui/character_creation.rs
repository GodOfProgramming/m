use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game::{GameState, StartGameEvent};

#[derive(Default, Resource)]
pub struct CharacterCreationMenu {
  name: String,
}

pub fn on_enter(mut commands: Commands) {
  commands.insert_resource(CharacterCreationMenu::default())
}

pub fn on_update(
  mut next_state: ResMut<NextState<GameState>>,
  mut event_writer: EventWriter<StartGameEvent>,
  mut contexts: EguiContexts,
  mut menu: ResMut<CharacterCreationMenu>,
) {
  egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    if ui.button("New Character").clicked() {
      next_state.set(GameState::CharacterCreate);
    }

    ui.separator();

    ui.text_edit_singleline(&mut menu.name);

    if ui.button("Create").clicked() {
      event_writer.send(StartGameEvent {
        name: menu.name.clone(),
      });
      next_state.set(GameState::StartGame)
    }
  });
}

pub fn on_exit(mut commands: Commands) {
  commands.remove_resource::<CharacterCreationMenu>();
}
