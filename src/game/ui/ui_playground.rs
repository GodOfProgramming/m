use bevy::prelude::*;

use crate::storage::SystemInformation;

#[derive(Resource)]
pub struct UiPlayground {
  handle: Entity,
}

#[derive(Component)]
pub struct UiPlaygroundButton;

#[derive(Component)]
pub struct UiPlaygroundText;

pub fn on_enter(
  mut commands: Commands,
  entities: Query<Entity>,
  mut sys_info: ResMut<SystemInformation>,
) {
  for entity in entities.iter() {
    commands.entity(entity).despawn();
  }

  sys_info.current_camera = Some(commands.spawn(Camera2dBundle::default()).id());
  let mut handle: Option<Entity> = None;
  commands
    .spawn((
      ButtonBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          margin: UiRect::all(Val::Px(2.0)),
          ..default()
        },
        background_color: BackgroundColor(Color::RED),
        ..default()
      },
      UiPlaygroundButton,
    ))
    .with_children(|parent| {
      handle = Some(
        parent
          .spawn(TextBundle::from_section(
            "Placeholder",
            TextStyle {
              font_size: 40.0,
              ..default()
            },
          ))
          .id(),
      );
    });
  let playground = UiPlayground {
    handle: handle.unwrap(),
  };
  commands.insert_resource(playground);
}

pub fn on_update(
  playground: Res<UiPlayground>,
  interaction_query: Query<
    (&Interaction, &UiPlaygroundButton),
    (Changed<Interaction>, With<Button>),
  >,
) {
}

pub fn on_exit(mut commands: Commands, playground: Res<UiPlayground>) {
  commands.entity(playground.handle).despawn();
  commands.remove_resource::<UiPlayground>();
}
