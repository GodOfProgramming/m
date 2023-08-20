pub mod ui;

use bevy::{app::AppExit, prelude::*, tasks::Task};

use crate::{
  fatal_error,
  storage::{
    saves::{Attributes as SavedAttributes, SaveData, SaveDataBuilder},
    SystemInformation,
  },
};

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

pub fn startup(
  mut commands: Commands,
  mut next_state: ResMut<NextState<GameState>>,
  mut sys_info: ResMut<SystemInformation>,
) {
  sys_info.current_camera = Some(commands.spawn(Camera2dBundle::default()).id());
  next_state.set(GameState::MainMenu);
}

pub fn global_input_handler(
  kbd: Res<Input<KeyCode>>,
  mut exit: EventWriter<AppExit>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  if kbd.just_pressed(KeyCode::F9) {
    next_state.set(GameState::UiPlayground);
  }

  if kbd.just_pressed(KeyCode::Escape) {
    exit.send(AppExit);
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
    mut event: EventReader<Self>,
    sys_info: Res<SystemInformation>,
    mut event_writer: EventWriter<SaveDataLoadedEvent>,
  ) {
    if let Some(event) = event.into_iter().next() {
      let file_path = sys_info.game_saves_path.join(format!("{}.ms", event.name));
      if file_path.exists() {
        // load existing save

        if let Ok(save_data) = std::fs::read(file_path) {
          if let Ok(save_data) = bincode::deserialize::<SaveData>(&save_data) {
            event_writer.send(SaveDataLoadedEvent(save_data));
          } else {
            fatal_error("player save data is corrupt");
          }
        } else {
          fatal_error("could not read save data file");
        }
      } else {
        // create new character
        event_writer.send(SaveDataLoadedEvent(
          SaveDataBuilder::new()
            .name(event.name.clone())
            .attributes(SavedAttributes::default())
            .build(),
        ));
      }
    } else {
      fatal_error("began game with no character")
    }
  }
}

#[derive(Event)]
pub struct SaveDataLoadedEvent(SaveData);

impl SaveDataLoadedEvent {
  pub fn handle(
    mut commands: Commands,
    mut sys_info: ResMut<SystemInformation>,
    mut event_reader: EventReader<Self>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    for event in event_reader.iter() {
      let save_data = event.data();
      const PLAYER_SIZE: f32 = 100.0;
      if let Some(entity) = sys_info.current_camera {
        commands.entity(entity).despawn();
      }
      sys_info.current_camera = Some(
        commands
          .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, PLAYER_SIZE * 5.0)
              .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
          })
          .id(),
      );
      commands.spawn((
        PlayerCharacter,
        Name(save_data.name.clone()),
        Attributes::from(save_data.attributes.clone()),
        PbrBundle {
          mesh: meshes.add(shape::Cube::new(PLAYER_SIZE).into()),
          material: materials.add(Color::PURPLE.into()),
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_SIZE / 2.0)),
          ..default()
        },
      ));
      commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(PLAYER_SIZE * 5.0).into()),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 90.0_f32.to_radians())),
        ..default()
      });
      next_state.set(GameState::Gameplay);
      break;
    }
  }

  fn data(&self) -> &SaveData {
    &self.0
  }
}

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Attributes {
  // health
  vitality: u32,
  // resistances
  endurance: u32,
  // attack magnitude
  strength: u32,
  // attack speed
  dexterity: u32,
  // movement
  agility: u32,
  // magic damage
  intelligence: u32,
  // magic efficiency
  wisdom: u32,
  // spell memorization
  mind: u32,
}

impl Attributes {
  fn move_speed(&self) -> f32 {
    self.agility as f32
  }
}

impl From<SavedAttributes> for Attributes {
  fn from(save: SavedAttributes) -> Self {
    Self {
      vitality: save.vitality,
      endurance: save.endurance,
      strength: save.strength,
      dexterity: save.dexterity,
      agility: save.agility,
      intelligence: save.intelligence,
      wisdom: save.wisdom,
      mind: save.mind,
    }
  }
}

pub fn player_movement_system(
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut query: Query<(&mut Transform, &Attributes), With<PlayerCharacter>>,
) {
  let player_query = query.single_mut();
  let attributes = player_query.1;
  let mut transform = player_query.0;

  let mut movement = Vec2::default();

  let mut moved = false;
  // TODO why does two opposing key inputs make the player vanish?
  if keyboard_input.pressed(KeyCode::W) {
    movement.y += 1.0;
    moved = true;
  } else if keyboard_input.pressed(KeyCode::S) {
    movement.y -= 1.0;
    moved = true;
  }

  if keyboard_input.pressed(KeyCode::A) {
    movement.x -= 1.0;
    moved = true;
  } else if keyboard_input.pressed(KeyCode::D) {
    movement.x += 1.0;
    moved = true;
  }

  if moved {
    let movement = movement.normalize() * attributes.move_speed() * time.delta().as_millis() as f32;
    transform.translation += Vec3::new(movement.x, movement.y, 0.0);
  }
}

pub fn focus_camera_system(
  time: Res<Time>,
  mut query: ParamSet<(
    Query<&mut Transform, With<Camera3d>>,
    Query<(&Transform, &Attributes), With<PlayerCharacter>>,
  )>,
) {
  let player_query = query.p1();
  let char_pos = {
    let player_transform = player_query.single().0;
    Vec2::new(
      player_transform.translation.x,
      player_transform.translation.y,
    )
  };

  let move_speed = {
    let player_attributes = player_query.single().1;
    player_attributes.move_speed()
  };

  let mut cam_query = query.p0();
  let mut cam_transform = cam_query.single_mut();

  let cam_pos = Vec2::new(cam_transform.translation.x, cam_transform.translation.y);
  let dist = cam_pos.distance(char_pos);
  if dist > 0.0 {
    const MAX_DIST: f32 = 256.0; // TODO arbitrary, figure out how to calculate dynamically (if needed?)
    let direction = (char_pos - cam_pos).normalize();
    let modifier = move_speed * time.delta().as_millis() as f32;
    let direction = direction * modifier * f32::min(dist, MAX_DIST) / MAX_DIST;
    let old_coords = Vec2::new(cam_transform.translation.x, cam_transform.translation.y);
    let mut new_coords = old_coords + direction;
    let z_pos = cam_transform.translation.z;

    let bounds = Rect::new(old_coords.x, old_coords.y, new_coords.x, new_coords.y);
    if bounds.contains(char_pos) {
      new_coords = char_pos;
    }

    cam_transform.translation = Vec3::new(new_coords.x, new_coords.y, z_pos);
    cam_transform.look_at(Vec3::new(new_coords.x, new_coords.y, 0.0), Vec3::Y);
  }
}
