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

impl FlyCamSettings {
    pub fn new(sensitivity: f32, move_speed: f32) -> Self {
        Self {
            sensitivity,
            move_speed,
        }
    }
}

#[derive(Component)]
pub struct CameraMarker;

pub struct FlyCamPlugin;
impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlyCamSettings>();
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
    mut query: Query<&mut Transform, With<CameraMarker>>,
) {
    let mut transform = query.single_mut();

    let mut delta = Vec3::ZERO;

    let forward = *transform.local_z();
    let right = *transform.local_x();
    if keyboard_input.pressed(KeyCode::KeyW) {
        delta -= forward;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta += forward;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta += right;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        delta -= right;
    }
    delta = delta.normalize_or_zero();
    if keyboard_input.pressed(KeyCode::Space) {
        delta.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        delta.y -= 1.0;
    }

    transform.translation += delta.normalize_or_zero() * settings.move_speed * time.delta_seconds();
}
