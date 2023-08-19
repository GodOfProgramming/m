pub mod ui;

use bevy::{app::AppExit, ecs::world::EntityMut, prelude::*};

use crate::storage::SystemInformation;

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

pub fn on_enter(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
  commands.spawn(Camera2dBundle::default());
  next_state.set(GameState::MainMenu);
}

pub fn global_input_handler(
  kbd: Res<Input<KeyCode>>,
  mouse: Res<Input<MouseButton>>,
  mut chars: EventReader<ReceivedCharacter>,
  mut exit: EventWriter<AppExit>,
  sys_info: Res<SystemInformation>,
  mut next_state: ResMut<NextState<GameState>>,
  mut entities: Query<&mut Text>,
) {
  if kbd.just_pressed(KeyCode::F9) {
    next_state.set(GameState::UiPlayground);
  }

  if kbd.just_pressed(KeyCode::Escape) {
    exit.send(AppExit);
  }

  if let Some(focus) = &sys_info.focused_entity {
    for c in chars.into_iter() {
      debug!("got {}", c.char);
      (focus.on_chars_received)(focus.handle, c.char, &mut entities);
    }
  }
}
