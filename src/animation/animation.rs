use bevy::prelude::*;


#[derive(Component)]
pub struct AnimationSprite {
    pub frame_count: usize,
    pub current_frame: usize,
    pub frame_duration: f32,
    pub elapsed_time: f32,
}