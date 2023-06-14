use bevy::prelude::*;
use super::grid::*;

pub const LIVE_CELL_COLOR: Color = Color::rgb(1., 1., 1.);
pub const DEAD_CELL_COLOR: Color = Color::rgb(0., 0., 0.);

pub const CELL_SIZE: Vec2 = Vec2::new(16., 16.);

pub const CELL_LAYER: f32 = 1.;

pub const CELL_GAP: f32 = 2.;

pub const CELL_PER_ROW: u8 = 71;
pub const CELL_PER_COLUMN: u8 = 30;

pub const CELL_ORIGIN: Vec3 = Vec3::new(-GRID_WIDTH / 2. + CELL_SIZE.x / 2. + CELL_GAP, 
                                        GRID_HEIGHT / 2. - CELL_SIZE.x / 2. - CELL_GAP, 
                                        CELL_LAYER);