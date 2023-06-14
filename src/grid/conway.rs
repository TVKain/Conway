use super::cell::*;
use super::grid::*;
use crate::animation::animation::*;
use bevy::prelude::*;

use crate::config::cell::*;

pub fn conway_grid_system(
    time: Res<Time>,
    mut logical_grid: ResMut<LogicalGrid>,
    mut query: Query<(&mut Sprite, &mut Cell, &mut AnimationSprite), With<Cell>>,
) {
    let clone = logical_grid.clone();
    for row in logical_grid.unwrap() {
        for cell in row {
            let live_neighbours = live_neighbours_count(&clone, cell.coordinate);

            match cell.state {
                CellState::Live => {
                    if live_neighbours == 2 || live_neighbours == 3 {
                        cell.next_state = CellState::Live;
                    } else {
                        cell.next_state = CellState::Dead;
                    }
                }
                CellState::Dead => {
                    if live_neighbours == 3 {
                        cell.next_state = CellState::Live;
                    } else {
                        cell.next_state = CellState::Dead;
                    }
                }
            }
        }
    }

    for row in logical_grid.unwrap() {
        for cell in row {
            cell.state = cell.next_state;
        }
    }

    for (mut sprite, mut cell, mut animation) in query.iter_mut() {
        animation.elapsed_time += time.delta_seconds();

        if animation.elapsed_time >= animation.frame_duration {
            let logical_cell = logical_grid.get_cell(cell.coordinate);
            cell.state = logical_cell.state;
            match logical_cell.state {
                CellState::Live => {
                    sprite.color = LIVE_CELL_COLOR;
                }
                CellState::Dead => {
                    sprite.color = DEAD_CELL_COLOR;
                }
            }

            animation.current_frame = (animation.current_frame + 1) % animation.frame_count;
            animation.elapsed_time = 0.0;
        }
    }
}

fn live_neighbours_count(logical_grid: &LogicalGrid, cell_coordinate: IVec2) -> u8 {
    let mut live_neighbours = 0;

    let neighbours = get_neighbours(cell_coordinate);

    for neighbour in neighbours {
        let neighbour_state = logical_grid.get_cell(neighbour).state;

        match neighbour_state {
            CellState::Live => live_neighbours += 1,
            _ => (),
        }
    }

    live_neighbours
}

fn get_neighbours(cell_coordinate: IVec2) -> Vec<IVec2> {
    let offsets: Vec<IVec2> = vec![
        (-1, -1).into(),
        (-1, 0).into(),
        (-1, 1).into(),
        (0, -1).into(),
        (0, 1).into(),
        (1, -1).into(),
        (1, 0).into(),
        (1, 1).into(),
    ];

    let mut neighbours: Vec<IVec2> = vec![];

    for offset in offsets {
        let current = cell_coordinate + offset;
        if current.x < 0 || current.y < 0 {
            continue;
        }

        if current.x >= (CELL_PER_COLUMN as i32) || current.y >= (CELL_PER_ROW as i32) {
            continue;
        }

        neighbours.push(current);
    }

    neighbours
}
