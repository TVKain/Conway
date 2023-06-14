use bevy::ui::*;
use bevy::prelude::*;

use super::{window::*, grid::GRID_HEIGHT};

pub const UI_SIZE: Size = Size::new(Val::Percent(100.0), Val::Px(WINDOW_HEIGHT - GRID_HEIGHT));

pub const BUTTON_SIZE: Size = Size::new(Val::Px(128.), Val::Px(64.));

pub const START_BUTTON_COLOR: Color = Color::rgb(0.08, 0.83, 0.52);
pub const START_BUTTON_COLOR_HOVER: Color = Color::rgb(0.094, 0.929, 0.58);
pub const START_BUTTON_COLOR_CLICKED: Color = Color::rgb(0.075, 0.702, 0.439);

pub const RESET_BUTTON_COLOR: Color = Color::rgb(0.9, 0.2, 0.32);
pub const RESET_BUTTON_COLOR_HOVER: Color = Color::rgb(0.94, 0.35, 0.4);
pub const RESET_BUTTON_COLOR_CLICKED: Color = Color::rgb(0.8, 0.1, 0.25);