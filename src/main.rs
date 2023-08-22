mod game;
mod storage;

use bevy::{
  log::{Level, LogPlugin},
  prelude::*,
  window::WindowResolution,
};
use bevy_egui::EguiPlugin;
use dialog::DialogBox;
use game::{
  ui::{
    main_menu,
    settings_menu::{self, SaveSettingsEvent},
    ui_playground, WindowEvent,
  },
  GameState,
};
use platform_dirs::AppDirs;
use std::error::Error;
use storage::{Settings, SystemInformation};

use crate::game::{
  ui::{character_creation, character_selection},
  SaveDataLoadedEvent, StartGameEvent,
};

const GAME_NAME: &'static str = "M";

fn main() -> Result<(), Box<dyn Error>> {
  let game_dir = AppDirs::new(Some(GAME_NAME), true)
    .map(|d| d.data_dir)
    .ok_or("unable to acquire data directory, cannot save anything")?;

  let game_saves_path = game_dir.join("saves");
  let settings_path = game_dir.join("settings.toml");

  println!(
    "Saving all data to {}",
    game_dir.as_os_str().to_string_lossy()
  );

  let settings = Settings::load_or_default(&settings_path);
  let sys_info = SystemInformation::new(game_saves_path, settings_path, settings);

  App::new()
    .add_plugins((
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: GAME_NAME.to_string(),
            mode: sys_info.settings.window.mode.into(),
            resolution: WindowResolution::new(
              sys_info.settings.window.width as f32,
              sys_info.settings.window.height as f32,
            ),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            decorations: true,
            ..default()
          }),
          ..default()
        })
        .set(LogPlugin {
          level: if cfg!(debug_assertions) {
            Level::DEBUG
          } else {
            Level::INFO
          },
          ..default()
        }),
      EguiPlugin,
    ))
    .add_state::<GameState>()
    .add_event::<WindowEvent>()
    .add_event::<SaveSettingsEvent>()
    .add_event::<StartGameEvent>()
    .add_event::<SaveDataLoadedEvent>()
    // global
    .add_systems(Startup, game::startup)
    .add_systems(Update, game::global_input_handler)
    // main menu
    .add_systems(OnEnter(GameState::MainMenu), main_menu::on_enter)
    .add_systems(
      Update,
      main_menu::on_update.run_if(in_state(GameState::MainMenu)),
    )
    .add_systems(OnExit(GameState::MainMenu), main_menu::on_exit)
    // character select
    .add_systems(
      OnEnter(GameState::CharacterSelect),
      character_selection::on_enter,
    )
    .add_systems(
      Update,
      character_selection::on_update.run_if(in_state(GameState::CharacterSelect)),
    )
    .add_systems(
      OnExit(GameState::CharacterSelect),
      character_selection::on_exit,
    )
    // character create
    .add_systems(
      OnEnter(GameState::CharacterCreate),
      character_creation::on_enter,
    )
    .add_systems(
      Update,
      character_creation::on_update.run_if(in_state(GameState::CharacterCreate)),
    )
    .add_systems(
      OnExit(GameState::CharacterCreate),
      character_creation::on_exit,
    )
    // initialize game
    .add_systems(OnEnter(GameState::StartGame), StartGameEvent::handle)
    .add_systems(
      Update,
      SaveDataLoadedEvent::handle.run_if(in_state(GameState::StartGame)),
    )
    // play game
    .add_systems(OnEnter(GameState::Gameplay), game::gameplay_startup)
    .add_systems(
      Update,
      (
        game::player_top_down_movement_system,
        game::focus_top_down_camera_system,
        // game::player_first_person_movement_system,
        // game::focus_first_person_camera_system,
      )
        .chain()
        .run_if(in_state(GameState::Gameplay)),
    )
    // settings
    .add_systems(OnEnter(GameState::SettingsMenu), settings_menu::on_enter)
    .add_systems(
      Update,
      (
        settings_menu::on_update,
        SaveSettingsEvent::handler,
        WindowEvent::handler,
      )
        .run_if(in_state(GameState::SettingsMenu)),
    )
    .add_systems(OnExit(GameState::SettingsMenu), settings_menu::on_exit)
    // debug
    .add_systems(OnEnter(GameState::UiPlayground), ui_playground::on_enter)
    .add_systems(
      Update,
      ui_playground::on_update.run_if(in_state(GameState::UiPlayground)),
    )
    .add_systems(OnExit(GameState::UiPlayground), ui_playground::on_exit)
    .insert_resource(sys_info)
    .run();

  Ok(())
}

pub fn fatal_error(msg: &str) -> ! {
  error!("{}", msg);
  dialog::Message::new(msg)
    .title("Fatal Error")
    .show()
    .expect("failed to show dialog box");
  panic!("{}", msg);
}
