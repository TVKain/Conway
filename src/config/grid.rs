use bevy::prelude::*;
use super::cell::*;
use super::window::*;

pub const GRID_COLOR: Color = Color::ANTIQUE_WHITE;

pub const GRID_WIDTH: f32 = (CELL_PER_ROW as f32) * (CELL_SIZE.x + CELL_GAP) + CELL_GAP;
pub const GRID_HEIGHT: f32 = (CELL_PER_COLUMN as f32) * (CELL_SIZE.x + CELL_GAP) + CELL_GAP;

pub const GRID_SIZE: Vec2 = Vec2::new(GRID_WIDTH, GRID_HEIGHT);

pub const GRID_LAYER: f32 = 0.;
pub const GRID_TRANSLATION: Vec3 = Vec3::new(0., -(WINDOW_HEIGHT - GRID_HEIGHT) / 2., GRID_LAYER);
