pub mod ui;

use bevy::{
  app::AppExit,
  input::mouse::{MouseMotion, MouseWheel},
  prelude::*,
  tasks::Task,
  window::CursorGrabMode,
};

use crate::{
  fatal_error,
  storage::{
    saves::{Attributes as SavedAttributes, SaveData, SaveDataBuilder},
    SystemInformation,
  },
};

const PLAYER_SIZE: f32 = 100.0;
const DEADZONE: f32 = 0.15;
const MOUSE_SENSITIVITY: f32 = 0.1;
const UP: Vec3 = Vec3::Z;

#[derive(Resource)]
pub struct GameInfo {
  compute_front: fn(Vec3) -> Vec3, // current front -> new front
  focus_camera: fn(Vec3, Vec3) -> (Vec3, Vec3), // player pos, direction -> cam pos, cam focus
}

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
      if let Some(entity) = sys_info.current_camera {
        commands.entity(entity).despawn();
      }
      sys_info.current_camera = Some(
        commands
          .spawn((
            Camera3dBundle {
              transform: Transform::from_xyz(0.0, 0.0, PLAYER_SIZE * 5.0)
                .looking_at(Vec3::ZERO, UP),
              ..default()
            },
            Front::default(),
            EulerAngles {
              yaw: 90.0,
              pitch: 0.0,
              roll: 0.0,
            },
          ))
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
      commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(PLAYER_SIZE * 5.0).into()),
        material: materials.add(Color::RED.into()),
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

#[derive(Component)]
pub struct Front {
  direction: Vec3,
}

impl Default for Front {
  fn default() -> Self {
    Self { direction: Vec3::Z }
  }
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

pub fn on_enter(mut commands: Commands, mut windows: Query<&mut Window>) {
  let mut window = windows.single_mut();
  window.cursor.grab_mode = CursorGrabMode::Locked;
  window.cursor.visible = false;
  commands.insert_resource(GameInfo {
    compute_front: player_first_person_front,
    focus_camera: cam_first_person_target_fn,
  });
}

pub fn on_exit(mut commands: Commands) {
  commands.remove_resource::<GameInfo>();
}

pub fn player_first_person_front(front: Vec3) -> Vec3 {
  front
}

pub fn player_third_person_front(front: Vec3) -> Vec3 {
  Vec3::new(front.x, front.y, 0.0).normalize()
}

pub fn player_movement_system(
  keyboard_input: Res<Input<KeyCode>>,
  gamepads: Res<Gamepads>,
  gamepad_buttons: Res<Input<GamepadButton>>,
  gamepad_axis: Res<Axis<GamepadAxis>>,
  time: Res<Time>,
  mut game_info: ResMut<GameInfo>,
  mut query: ParamSet<(
    Query<(&mut Transform, &Attributes), With<PlayerCharacter>>,
    Query<&Front, With<Camera3d>>,
  )>,
) {
  // actions

  let should_swap_cam = keyboard_input.just_pressed(KeyCode::F3)
    || gamepads
      .iter()
      .next()
      .map(|gp| gamepad_buttons.just_pressed(GamepadButton::new(gp, GamepadButtonType::Select)))
      .unwrap_or_default();
  if should_swap_cam {
    game_info.compute_front = if game_info.compute_front == player_first_person_front {
      player_third_person_front
    } else {
      player_first_person_front
    };

    game_info.focus_camera = if game_info.focus_camera == cam_first_person_target_fn {
      cam_third_person_target_fn
    } else {
      cam_first_person_target_fn
    };
  }

  let front = query.p1().single().direction;
  let front = (game_info.compute_front)(front);
  let mut player_query = query.p0();
  let move_speed = player_query.single().1.move_speed();
  let mut transform = player_query.single_mut().0;

  let mut movement = Vec3::default();

  let mut moved = false;
  if keyboard_input.pressed(KeyCode::W) {
    movement += front;
    moved = true;
  } else if keyboard_input.pressed(KeyCode::S) {
    movement -= front;
    moved = true;
  }

  if keyboard_input.pressed(KeyCode::A) {
    movement -= front.cross(UP);
    moved = true;
  } else if keyboard_input.pressed(KeyCode::D) {
    movement += front.cross(UP);
    moved = true;
  }

  for gamepad in gamepads.iter() {
    let (x, y) = (
      gamepad_axis
        .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
        .unwrap_or_default(),
      gamepad_axis
        .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
        .unwrap_or_default(),
    );

    if x.abs() > DEADZONE {
      movement += front.cross(UP) * x;
      moved = true;
    }

    if y.abs() > DEADZONE {
      movement += front * y;
      moved = true;
    }

    break;
  }

  if moved {
    let movement = movement.normalize() * move_speed * time.delta().as_millis() as f32;
    transform.translation += movement;
  }
}

#[derive(Default, Component)]
pub struct EulerAngles {
  yaw: f32,
  pitch: f32,
  #[allow(unused)]
  roll: f32,
}

pub fn cam_first_person_target_fn(player_pos: Vec3, direction: Vec3) -> (Vec3, Vec3) {
  (player_pos, player_pos + direction)
}

pub fn cam_third_person_target_fn(player_pos: Vec3, direction: Vec3) -> (Vec3, Vec3) {
  (player_pos - (direction * PLAYER_SIZE * 5.0), player_pos)
}

pub fn focus_camera_system(
  mut mouse_motion: EventReader<MouseMotion>,
  gamepads: Res<Gamepads>,
  gamepad_input: Res<Axis<GamepadAxis>>,
  game_info: Res<GameInfo>,
  mut query: ParamSet<(
    Query<(&mut Transform, &mut Front, &mut EulerAngles), With<Camera3d>>,
    Query<&Transform, With<PlayerCharacter>>,
  )>,
) {
  let player_pos = query.p1().single().translation;

  let mut cam_query = query.p0();
  let cam_query = cam_query.single_mut();

  let (mouse_x, mouse_y) = mouse_motion
    .iter()
    .map(|motion| motion.delta)
    .reduce(|c, n| c + n)
    .map(|offsets| (offsets.x * MOUSE_SENSITIVITY, offsets.y * MOUSE_SENSITIVITY))
    .unwrap_or_default();

  let (gamepad_x, gamepad_y) = gamepads
    .iter()
    .next()
    .map(|gp| {
      (
        gamepad_input
          .get(GamepadAxis::new(gp, GamepadAxisType::RightStickX))
          .unwrap_or_default(),
        gamepad_input
          .get(GamepadAxis::new(gp, GamepadAxisType::RightStickY))
          .unwrap_or_default(),
      )
    })
    .map(|(x, y)| {
      (
        if x.abs() > DEADZONE { x } else { 0.0 },
        if y.abs() > DEADZONE { y } else { 0.0 },
      )
    })
    .unwrap_or_default();

  let (yaw_rad, pitch_rad) = {
    // set cam rotation
    let mut euler_angles = cam_query.2;

    euler_angles.yaw -= mouse_x + gamepad_x;
    euler_angles.pitch -= mouse_y - gamepad_y;

    euler_angles.yaw %= 360.0;

    euler_angles.pitch = euler_angles.pitch.clamp(-89.0, 89.0);
    (
      euler_angles.yaw.to_radians(),
      euler_angles.pitch.to_radians(),
    )
  };

  let yaw_sin = yaw_rad.sin();
  let pitch_sin = pitch_rad.sin();

  let yaw_cos = yaw_rad.cos();
  let pitch_cos = pitch_rad.cos();

  let direction = Vec3::new(pitch_cos * yaw_cos, pitch_cos * yaw_sin, pitch_sin).normalize();

  // set cam front
  let mut front = cam_query.1;
  front.direction = direction;

  let mut cam_transform = cam_query.0;
  let (cam_pos, cam_focus) = (game_info.focus_camera)(player_pos, direction);

  // set cam position
  cam_transform.translation = cam_pos;

  // set cam look
  cam_transform.look_at(cam_focus, UP);
}
