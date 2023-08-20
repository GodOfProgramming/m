use super::WindowEvent;
use crate::{
  game::GameState,
  storage::{Settings, SystemInformation, WindowMode},
};
use bevy::prelude::*;
use bevy_egui::{
  egui::{self, Align2, Color32, Frame, Ui},
  EguiContexts,
};
use phf::{phf_map, Map};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

static RESOLUTIONS: Map<&'static str, UVec2> = phf_map! {
  "1280x720" => UVec2::new(1280, 720),
  "1920x1080" => UVec2::new(1920, 1080),
};

#[derive(Default, Resource)]
pub struct SettingsMenu {
  menu_map: BTreeMap<
    &'static str,
    Box<dyn FnMut(&mut Ui, &mut Settings, &mut EventWriter<WindowEvent>) + Send + Sync>,
  >,
}

#[derive(Event)]
pub struct SaveSettingsEvent;

impl SaveSettingsEvent {
  pub fn handler(
    sys_info: Res<SystemInformation>,
    mut event_reader: EventReader<SaveSettingsEvent>,
  ) {
    for _ in event_reader.into_iter() {
      if let Err(e) = sys_info.save_settings() {
        warn!("failed to save settings: {}", e);
      }
    }
  }
}

pub fn on_enter(mut commands: Commands) {
  let mut menu = SettingsMenu::default();

  menu.menu_map.insert(
    "Window Size",
    Box::new(|ui, settings, window_event_writer| {
      ui.collapsing(
        format!("{}x{}", settings.window.width, settings.window.height),
        |ui| {
          for (display, res) in RESOLUTIONS.entries() {
            if ui.button(*display).clicked() {
              window_event_writer.send(WindowEvent::Resize(res.x, res.y));
            }
          }
        },
      );
    }),
  );
  menu.menu_map.insert(
    "Window Mode",
    Box::new(|ui, settings, window_event_writer| {
      ui.collapsing(settings.window.mode.to_string(), |ui| {
        for mode in WindowMode::iter() {
          if ui.button(mode.to_string()).clicked() {
            window_event_writer.send(WindowEvent::ModeChange(mode));
          }
        }
      });
    }),
  );
  commands.insert_resource(menu);
}

pub fn on_update(
  mut next_state: ResMut<NextState<GameState>>,
  mut sys_info: ResMut<SystemInformation>,
  mut contexts: EguiContexts,
  mut settings_menu: ResMut<SettingsMenu>,
  mut window_resize_event_writer: EventWriter<WindowEvent>,
  mut save_settings_event_writer: EventWriter<SaveSettingsEvent>,
) {
  egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
    ui.heading("Settings");

    for (key, value) in settings_menu.menu_map.iter_mut() {
      ui.horizontal(|ui| {
        ui.label(key.to_string());
        value(ui, &mut sys_info.settings, &mut window_resize_event_writer);
      });
    }

    ui.horizontal(|ui| {
      if ui.button("Back").clicked() {
        next_state.set(GameState::MainMenu);
      }

      if ui.button("Save").clicked() {
        save_settings_event_writer.send(SaveSettingsEvent);
      }
    });
  });
  egui::CentralPanel::default()
    .frame(Frame::default().fill(Color32::BLACK))
    .show(contexts.ctx_mut(), |_ui| {});
}

pub fn on_exit(mut commands: Commands) {
  commands.remove_resource::<SettingsMenu>();
}
