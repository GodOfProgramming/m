use bevy::{
  ecs::{entity::EntityMap, world::EntityMut},
  prelude::*,
};

use crate::storage::{Focus, SystemInformation};

#[derive(Resource)]
pub struct UiPlayground {
  handle: Entity,
}

#[derive(Component)]
pub struct UiPlaygroundButton;

#[derive(Component)]
pub struct UiPlaygroundText;

pub fn on_enter(mut commands: Commands) {
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
  mut sys_info: ResMut<SystemInformation>,
  interaction_query: Query<
    (&Interaction, &UiPlaygroundButton),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, _button) in interaction_query.into_iter() {
    match interaction {
      Interaction::Pressed => {
        debug!("clicked button");
        sys_info.focused_entity = Some(Focus {
          handle: playground.handle,
          on_chars_received: |handle: Entity, c: char, entities: &mut Query<&mut Text>| {
            if let Ok(mut text) = entities.get_component_mut::<Text>(handle) {
              let curr = &text.sections[0].value;
              debug!("curr text = {}", curr);
              text.sections[0].value = format!("{}{}", curr, c);
            } else {
              debug!("entity is not text");
            }
          },
        });
      }
      _ => (),
    }
  }
}

pub fn on_exit(mut commands: Commands, playground: Res<UiPlayground>) {
  commands.entity(playground.handle).despawn();
  commands.remove_resource::<UiPlayground>();
}
