use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

pub fn cursor_track_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<MousePosition>
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_position.0 = world_position;
    }
}