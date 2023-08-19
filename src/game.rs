pub mod ui;

use bevy::{
  app::AppExit,
  prelude::*,
  tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use serde::Deserialize;

use crate::{
  fatal_error,
  storage::{saves::SaveData, SystemInformation},
};

use ui::FocusedEntity;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Startup,
  StartGame,
  MainMenu,
  CharacterSelect,
  CharacterCreate,
  SettingsMenu,
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

#[derive(Component)]
pub struct LoadPlayer {
  task: Task<SaveData>,
}

#[derive(Event)]
pub struct StartGameEvent {
  name: String,
}

impl StartGameEvent {
  pub fn handle(
    mut commands: Commands,
    mut event: EventReader<StartGameEvent>,
    sys_info: Res<SystemInformation>,
    mut next_state: ResMut<NextState<GameState>>,
  ) {
    let thread_pool = AsyncComputeTaskPool::get();
    if let Some(event) = event.into_iter().next() {
      let file_path = sys_info.game_saves_path.join(format!("{}.ms", event.name));
      if file_path.exists() {
        // load existing save
        let task = thread_pool.spawn(async move {
          if let Ok(save_data) = std::fs::read(file_path) {
            if let Ok(player_data) = bincode::deserialize::<SaveData>(&save_data) {
              player_data
            } else {
              fatal_error("player save data is corrupt");
            }
          } else {
            fatal_error("could not read save data file");
          }
        });
        commands.spawn(LoadPlayer { task });
      } else {
        // create new character
        commands.spawn((PlayerCharacter, Name(event.name.clone())));
        next_state.set(GameState::Gameplay);
      }
    } else {
      fatal_error("began game with no character")
    }
  }
}

pub fn save_data_receiver(mut query: Query<&mut LoadPlayer>) -> SaveData {
  let mut load_player = query.single_mut();
  if let Some(save_data) = future::block_on(future::poll_once(&mut load_player.task)) {
    save_data
  } else {
    fatal_error("save data load order of operations error");
  }
}

pub fn spawn_player(
  In(save_data): In<SaveData>,
  mut commands: Commands,
  mut next_state: ResMut<NextState<GameState>>,
) {
  commands.spawn((PlayerCharacter, Name(save_data.name)));
  next_state.set(GameState::Gameplay);
}

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Name(String);
