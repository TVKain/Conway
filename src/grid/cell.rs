use bevy::prelude::*;

use crate::{config::cell::*, mouse_position::mouse_position::MousePosition};
use crate::grid::grid::*;

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Live,
    Dead,
}

#[derive(Component, Copy, Clone)]
pub struct Cell {
    pub state: CellState, 
    pub next_state: CellState, 
    pub coordinate: IVec2, 
}

impl Cell {
    pub fn new(state: CellState, next_state: CellState, coordinate: IVec2) -> Cell {
        Cell {
            state, 
            next_state,
            coordinate 
        }
    }
}

pub fn click_cell_system(
    mut logical_grid: ResMut<LogicalGrid>, 
    buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut query: Query<(&GlobalTransform, &mut Sprite, &mut Cell), With<Cell>>,
) {
    for (transform, mut sprite, mut cell) in query.iter_mut() {
        if cell_clicked(
            &buttons,
            &mouse_position,
            transform.translation().truncate(),
        ) {
            match &cell.state {
                CellState::Dead => {
                    sprite.color = LIVE_CELL_COLOR;
                    cell.state = CellState::Live;
                }
                CellState::Live => {
                    sprite.color = DEAD_CELL_COLOR;
                    cell.state = CellState::Dead;
                }
            }
            logical_grid.update_cell(*cell);
        }
    }
}

pub fn hover_cell_system(
    mut windows: Query<&mut Window>,
    mouse_position: Res<MousePosition>,
    query: Query<&GlobalTransform, With<Cell>>,
) {
    let mut window = windows.single_mut();
    for transform in query.iter() {
        if cell_hover(&mouse_position, transform.translation().truncate()) {
            window.cursor.icon = CursorIcon::Hand;
            break;
        } else {
            window.cursor.icon = CursorIcon::Arrow;
        }
    }
}

fn cell_hover(mouse_position: &Res<MousePosition>, sprite_position: Vec2) -> bool {
    mouse_position.0.distance(sprite_position) < 8.0 
}

fn cell_clicked(
    buttons: &Res<Input<MouseButton>>,
    mouse_position: &Res<MousePosition>,
    sprite_position: Vec2,
) -> bool {
    buttons.just_pressed(MouseButton::Left) && cell_hover(&mouse_position, sprite_position)
}
