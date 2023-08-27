pub fn player_top_down_movement_system(
  keyboard_input: Res<Input<KeyCode>>,
  gamepads: Res<Gamepads>,
  gamepad_input: Res<Axis<GamepadAxis>>,
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

  for gamepad in gamepads.iter() {
    let (x, y) = (
      gamepad_input
        .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
        .unwrap_or_default(),
      gamepad_input
        .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
        .unwrap_or_default(),
    );

    if x.abs() > DEADZONE {
      movement.x += x;
      moved = true;
    }

    if y.abs() > DEADZONE {
      movement.y += y;
      moved = true
    }

    break;
  }

  if moved {
    let movement = movement.normalize() * attributes.move_speed() * time.delta().as_millis() as f32;
    transform.translation += Vec3::new(movement.x, movement.y, 0.0);
  }
}

pub fn focus_top_down_camera_system(
  time: Res<Time>,
  mut scroll_wheel: EventReader<MouseWheel>,
  gamepads: Res<Gamepads>,
  gamepad_input: Res<Axis<GamepadAxis>>,
  mut query: ParamSet<(
    Query<&mut Transform, With<Camera3d>>,
    Query<(&Transform, &Attributes), With<PlayerCharacter>>,
  )>,
) {
  let player_query = query.p1();
  let (player_translate, char_pos) = {
    let player_translate = player_query.single().0.translation;
    let char_pos = Vec2::new(player_translate.x, player_translate.y);
    (player_translate, char_pos)
  };

  let move_speed = {
    let player_query = query.p1();
    let player_attributes = player_query.single().1;
    player_attributes.move_speed()
  };

  let mut cam_query = query.p0();
  let mut cam_transform = cam_query.single_mut();

  let cam_pos = Vec2::new(cam_transform.translation.x, cam_transform.translation.y);
  let dist = cam_pos.distance(char_pos);

  const ZPOS_SCALAR: f32 = 10.0; // TODO arbitrary

  let gamepad_y = gamepads
    .iter()
    .next()
    .map(|gp| gamepad_input.get(GamepadAxis::new(gp, GamepadAxisType::RightStickY)))
    .flatten()
    .map(|y| if y.abs() > DEADZONE { y } else { 0.0 })
    .unwrap_or_default();

  let z_pos = (cam_transform.translation.z
    - ((scroll_wheel
      .iter()
      .map(|e| e.y)
      .reduce(|c, n| c + n)
      .unwrap_or_default()
      + gamepad_y)
      * cam_transform.translation.distance(player_translate)
      / ZPOS_SCALAR))
    .clamp(PLAYER_SIZE * 3.0, PLAYER_SIZE * 15.0);

  let old_coords = Vec2::new(cam_transform.translation.x, cam_transform.translation.y);

  let new_coords = if dist > 0.0 {
    const MAX_DIST: f32 = 256.0; // TODO arbitrary, figure out how to calculate dynamically (if needed?)
    let direction = (char_pos - cam_pos).normalize();
    let modifier = move_speed * time.delta().as_millis() as f32;
    let direction = direction * modifier * f32::min(dist, MAX_DIST) / MAX_DIST;
    let mut new_coords = old_coords + direction;

    let bounds = Rect::new(old_coords.x, old_coords.y, new_coords.x, new_coords.y);
    if bounds.contains(char_pos) {
      new_coords = char_pos;
    }
    new_coords
  } else {
    old_coords
  };
  cam_transform.translation = Vec3::new(new_coords.x, new_coords.y, z_pos);
  cam_transform.look_at(player_translate, UP);
}
