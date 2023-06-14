use bevy::{prelude::*, window::WindowMode};
use crate::mouse_position::mouse_position::*;

use crate::config::window::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Draw,
    Running,
}

#[derive(Resource, Default)]
pub struct BackgroundSprite(String);

pub fn setup_system(mut commands: Commands, mut windows: Query<&mut Window>, mut clear_color: ResMut<ClearColor>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    setup_resource(&mut commands);
    setup_window(&mut windows.single_mut());
    setup_background(&mut clear_color);
}

fn setup_resource(commands: &mut Commands) {
    commands.insert_resource(MousePosition::default());
    commands.insert_resource(BackgroundSprite(String::from(BACKGROUND_SPRITE)));
}

fn setup_window(window: &mut Mut<Window>) {
    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT); 
    window.resizable = false;
    window.position = WindowPosition::Centered(MonitorSelection::Current);
    window.mode = WindowMode::Windowed;
    window.title = "Conway's game".into();
}

fn setup_background(clear_color: &mut ResMut<ClearColor>) {
    clear_color.0 = Color::ANTIQUE_WHITE;
}