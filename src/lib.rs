use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Resource)]
pub struct FlyCamSettings {
    pub sensitivity: f32,
    pub move_speed: f32,
}

impl Default for FlyCamSettings {
    fn default() -> Self {
        Self {
            sensitivity: 5.0,
            move_speed: 10.0,
        }
    }
}

#[derive(Resource)]
pub struct FlyCamKeybinds {
    move_forward: KeyCode,
    move_back: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode,
    move_up: KeyCode,
    move_down: KeyCode,
}

impl Default for FlyCamKeybinds {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_back: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::Space,
            move_down: KeyCode::ShiftLeft,
        }
    }
}

#[derive(Component)]
pub struct CameraMarker;

pub struct FlyCamPlugin;
impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlyCamSettings>();
        app.init_resource::<FlyCamKeybinds>();
        app.add_systems(Startup, lock_mouse);
        app.add_systems(Startup, setup_fly_cam);
        app.add_systems(Update, look_fly_cam);
        app.add_systems(Update, move_fly_cam);
    }
}

fn setup_fly_cam(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraMarker,
    ));
}

fn lock_mouse(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = query.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn look_fly_cam(
    time: Res<Time>,
    settings: Res<FlyCamSettings>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<CameraMarker>>,
) {
    for mut transform in &mut query {
        for motion in mouse_motion.read() {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            pitch -= (motion.delta.y * settings.sensitivity * time.delta_seconds()).to_radians();
            yaw -= (motion.delta.x * settings.sensitivity * time.delta_seconds()).to_radians();

            pitch = pitch.clamp(f32::to_radians(-89.0), f32::to_radians(89.0));

            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn move_fly_cam(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    settings: Res<FlyCamSettings>,
    keybinds: Res<FlyCamKeybinds>,
    mut query: Query<&mut Transform, With<CameraMarker>>,
) {
    let mut transform = query.single_mut();

    let mut delta = Vec3::ZERO;

    let back = *transform.local_z();
    let right = *transform.local_x();
    if keyboard_input.pressed(keybinds.move_forward) {
        delta -= back;
    }
    if keyboard_input.pressed(keybinds.move_back) {
        delta += back;
    }
    if keyboard_input.pressed(keybinds.move_right) {
        delta += right;
    }
    if keyboard_input.pressed(keybinds.move_left) {
        delta -= right;
    }
    if keyboard_input.pressed(keybinds.move_up) {
        delta.y += 1.0;
    }
    if keyboard_input.pressed(keybinds.move_down) {
        delta.y -= 1.0;
    }

    transform.translation += delta.normalize_or_zero() * settings.move_speed * time.delta_seconds();
}
