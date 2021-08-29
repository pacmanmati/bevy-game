use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Camera;

#[derive(Default)]
struct InputState {
    sensitivity: f32,
    pitch: f32,
    yaw: f32,
}

fn mouse_control(
    mut query: Query<(&Camera, &mut Transform)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut state: ResMut<InputState>,
    _kbd_input: Res<Input<KeyCode>>,
) {
    let (_, mut transform) = query
        .single_mut()
        .expect("There should only be one camera.");
    for event in mouse_motion_events.iter() {
        // info!("{:?}", event);
        let [x, y] = event.delta.to_array();
        state.yaw -= (x * state.sensitivity).to_radians();
        state.pitch -= (y * state.sensitivity).to_radians();
        state.pitch = state.pitch.clamp(-1.54, 1.54);
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, state.yaw) * Quat::from_axis_angle(Vec3::X, state.pitch);
        println!("{}", transform.rotation);
    }
}

pub struct FPSCameraPlugin {
    pub sensitivity: f32,
}

impl Plugin for FPSCameraPlugin {
    fn build(&self, app: &mut App) {
        let state = InputState {
            sensitivity: self.sensitivity,
            ..Default::default()
        };
        app.insert_resource(state).add_system(mouse_control);
    }
}
