pub mod main_menu;
pub mod settings_menu;
pub mod ui_playground;

use bevy::prelude::*;

use crate::storage::{SystemInformation, WindowMode};

#[derive(Event)]
pub enum WindowEvent {
  Resize(u32, u32),
  ModeChange(WindowMode),
}

impl WindowEvent {
  pub fn handler(
    mut windows: Query<&mut Window>,
    mut event_reader: EventReader<WindowEvent>,
    mut sys_info: ResMut<SystemInformation>,
  ) {
    let mut window = windows.single_mut();
    for event in event_reader.into_iter() {
      match *event {
        WindowEvent::Resize(width, height) => {
          sys_info.settings.window.width = width;
          sys_info.settings.window.height = height;
          window.resolution.set(width as f32, height as f32);
        }
        WindowEvent::ModeChange(mode) => {
          sys_info.settings.window.mode = mode;
          window.mode.set(Box::new(mode.to_bevy())).ok();
        }
      }
    }
  }
}

#[derive(Default, Resource)]
pub struct FocusedEntity {
  pub handle: Option<Entity>,
}
