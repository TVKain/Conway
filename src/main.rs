//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

use bevy::winit::WinitSettings;
use conway::grid::cell::*;
use conway::grid::conway::*;
use conway::grid::grid::*;
use conway::mouse_position::mouse_position::*;
use conway::setup::setup::*;
use conway::ui::button::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_systems((setup_system, grid_spawn_system, button_spawn_system))
        .add_system(cursor_track_system.run_if(in_state(AppState::Draw)))
        .add_systems((
            click_cell_system.run_if(in_state(AppState::Draw)),
            hover_cell_system.run_if(in_state(AppState::Draw)),
        ))
        .add_systems((
            start_button_event_system.run_if(in_state(AppState::Draw)),
            reset_button_event_system.run_if(in_state(AppState::Running)),
        ))
        .add_system(conway_grid_system.run_if(in_state(AppState::Running)))
        .run();
}
