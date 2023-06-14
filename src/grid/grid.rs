use crate::animation::animation::AnimationSprite;
use crate::config::cell::*;
use crate::config::grid::*;
use bevy::prelude::*;

use super::cell::*;

#[derive(Component, Default)]
pub struct Grid;

#[derive(Resource, Clone)]
pub struct LogicalGrid(pub Vec<Vec<Cell>>);

impl LogicalGrid {
    pub fn unwrap(&mut self) -> &mut Vec<Vec<Cell>> {
        &mut self.0
    }

    pub fn update_cell(&mut self, cell: Cell) {
        let old_cell = self
            .0
            .get_mut(cell.coordinate.x as usize)
            .unwrap()
            .get_mut(cell.coordinate.y as usize)
            .unwrap();

        *old_cell = cell;
    }

    pub fn get_cell(&self, coordinate: IVec2) -> Cell {
        self.0
            .get(coordinate.x as usize)
            .unwrap()
            .get(coordinate.y as usize)
            .unwrap()
            .clone()
    }
}

pub fn grid_spawn_system(mut commands: Commands) {
    let grid_entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(GRID_SIZE),
                color: GRID_COLOR,
                ..default()
            },
            transform: Transform {
                translation: GRID_TRANSLATION,
                ..default()
            },
            ..default()
        })
        .insert(Grid::default())
        .id();

    let mut logical_grid = LogicalGrid(vec![
        vec![
            Cell {
                state: CellState::Dead,
                next_state: CellState::Dead,
                coordinate: IVec2::new(0, 0),
            };
            CELL_PER_ROW as usize
        ];
        CELL_PER_COLUMN as usize
    ]);

    let mut current_cell_coord = CELL_ORIGIN;

    for i in 0..CELL_PER_COLUMN {
        for j in 0..CELL_PER_ROW {
            let cell_component = Cell::new(
                CellState::Dead,
                CellState::Live,
                IVec2::new(i.into(), j.into()),
            );

            logical_grid.update_cell(cell_component);

            let cell = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(CELL_SIZE),
                        color: DEAD_CELL_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: current_cell_coord,
                        ..default()
                    },
                    ..default()
                })
                .insert(cell_component)
                .insert(AnimationSprite {
                    frame_count: 30,
                    current_frame: 0,
                    frame_duration: 0.2,
                    elapsed_time: 0.,
                })
                .id();
            commands.entity(grid_entity).add_child(cell);

            current_cell_coord.x += CELL_GAP + CELL_SIZE.x;
        }
        current_cell_coord.y = current_cell_coord.y - (CELL_GAP + CELL_SIZE.x);
        current_cell_coord.x = CELL_ORIGIN.x;
    }

    commands.insert_resource(logical_grid);
}

pub fn reset_grid_system(
    logical_grid: &mut ResMut<LogicalGrid>,
    query: &mut Query<(&mut Sprite, &mut Cell), With<Cell>>,
) {
    for (mut sprite, mut cell) in query.iter_mut() {
        sprite.color = DEAD_CELL_COLOR;
        cell.state = CellState::Dead;
        logical_grid.update_cell(*cell);
    }
}
