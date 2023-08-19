pub mod ui;

use bevy::{app::AppExit, ecs::world::EntityMut, prelude::*};

use crate::storage::SystemInformation;

use self::ui::FocusedEntity;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Startup,
  MainMenu,
  PregameMenu,
  SettingsMenu,
  Loading,
  Gameplay,

  // debug
  UiPlayground,
}

pub fn startup(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
  commands.spawn(Camera2dBundle::default());
  commands.insert_resource(FocusedEntity::default());
  next_state.set(GameState::MainMenu);
}

pub fn global_input_handler(
  kbd: Res<Input<KeyCode>>,
  mouse: Res<Input<MouseButton>>,
  mut chars: EventReader<ReceivedCharacter>,
  mut exit: EventWriter<AppExit>,
  focus: Res<FocusedEntity>,
  mut next_state: ResMut<NextState<GameState>>,
  mut entities: Query<&mut Text>,
) {
  if kbd.just_pressed(KeyCode::F9) {
    next_state.set(GameState::UiPlayground);
  }

  if kbd.just_pressed(KeyCode::Escape) {
    exit.send(AppExit);
  }

  if let Some(handle) = focus.handle {
    for c in chars.into_iter().map(|c| c.char) {
      if let Ok(mut text) = entities.get_component_mut::<Text>(handle) {
        if c == '\x08' {
          text.sections[0].value.pop();
        } else {
          let curr = &text.sections[0].value;
          text.sections[0].value = format!("{}{}", curr, c);
        }
      }
    }
  }
}
