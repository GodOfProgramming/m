use crate::GameState;

use bevy::{app::AppExit, prelude::*};

#[derive(Resource)]
pub struct MainMenu {
  handle: Entity,
}

impl MainMenu {
  fn new(handle: Entity) -> Self {
    Self { handle }
  }
}

#[derive(Component)]
pub struct MainMenuButton(MainMenuButtonType);

impl MainMenuButton {
  fn kind(&self) -> MainMenuButtonType {
    self.0
  }
}

#[derive(Clone, Copy)]
enum MainMenuButtonType {
  Play,
  Settings,
  Exit,
}

pub fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
  let handle = commands
    .spawn(NodeBundle {
      style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
      },
      background_color: BackgroundColor(Color::BLACK),
      ..default()
    })
    .with_children(|parent| {
      // title
      parent.spawn(
        TextBundle::from_section(
          "M",
          TextStyle {
            font_size: 100.0,
            color: Color::WHITE,
            ..default()
          },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Center,
          justify_content: JustifyContent::FlexStart,
          ..default()
        }),
      );

      // menu buttons
      parent
        .spawn(NodeBundle {
          style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|parent| {
          const WIDTH: Val = Val::Percent(27.0);
          const HEIGHT: Val = Val::Percent(13.0);
          const BORDER: Val = Val::Px(2.0);
          const FONT_SIZE: f32 = 40.0;
          // play
          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  width: WIDTH,
                  height: HEIGHT,
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  margin: UiRect::all(BORDER),
                  ..default()
                },
                background_color: BackgroundColor(Color::PURPLE),
                ..default()
              },
              MainMenuButton(MainMenuButtonType::Play),
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                  font_size: FONT_SIZE,
                  color: Color::BLACK,
                  ..default()
                },
              ));
            });
          // settings
          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  width: WIDTH,
                  height: HEIGHT,
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  margin: UiRect::all(BORDER),
                  ..default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..default()
              },
              MainMenuButton(MainMenuButtonType::Settings),
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                  font_size: FONT_SIZE,
                  color: Color::BLACK,
                  ..default()
                },
              ));
            });
          // exit
          parent
            .spawn((
              ButtonBundle {
                style: Style {
                  width: WIDTH,
                  height: HEIGHT,
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  margin: UiRect::all(BORDER),
                  ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
              },
              MainMenuButton(MainMenuButtonType::Exit),
            ))
            .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                "Exit",
                TextStyle {
                  font_size: FONT_SIZE,
                  color: Color::BLACK,
                  ..default()
                },
              ));
            });
        });
    })
    .id();
  let menu = MainMenu::new(handle);
  commands.insert_resource(menu);
}

pub fn on_update(
  mut next_state: ResMut<NextState<GameState>>,
  mut exit: EventWriter<AppExit>,
  interaction_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
) {
  for (interaction, button) in interaction_query.into_iter() {
    match interaction {
      Interaction::Pressed => match button.kind() {
        MainMenuButtonType::Play => {
          next_state.set(GameState::CharacterSelect);
        }
        MainMenuButtonType::Settings => {
          next_state.set(GameState::SettingsMenu);
        }
        MainMenuButtonType::Exit => {
          exit.send(AppExit);
        }
      },
      _ => (),
    }
  }
}

pub fn on_exit(mut commands: Commands, menu: Res<MainMenu>) {
  commands.entity(menu.handle).despawn_recursive();
}
