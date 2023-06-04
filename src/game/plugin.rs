use crate::entities::*;
use crate::settings::local::Settings;
use bevy::{prelude::*, window::PrimaryWindow};
use rand;

use crate::game::renderer;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_random_field);
        app.add_system(field_iteration);
        app.add_system(renderer::render_cells);
    }
}

pub fn init_random_field(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
) {
    // Some calculations regarding a field size
    let window = window_query.get_single().unwrap();
    let block_size = settings.field_settings.block_size;
    let field_width_in_cells: usize = (window.width() / block_size) as usize;
    let field_height_in_cells: usize = (window.height() / block_size) as usize;

    // Initializing a field entity
    let mut field = field::Field {
        cells: vec![vec![Default::default(); field_width_in_cells]; field_height_in_cells],
    };

    for (x, y) in itertools::iproduct!(0..field_width_in_cells, 0..field_height_in_cells) {
        let new_cell_bundle = cell::CellBundle::new(
            (x as u32, y as u32),
            get_init_state((x as u32, y as u32), (field_width_in_cells as u32, field_height_in_cells as u32), 5),
            settings.clone(),
        );

        field.cells[x][y] = new_cell_bundle.cell().clone();
        commands.spawn(new_cell_bundle);
    }

    // Adding field entity
    commands.spawn(field);
    // Adding a timer
    commands.insert_resource(timer::FieldTimer::new(
        settings.field_settings.updating_frequency,
    ));
}

fn get_init_state((x, y): (u32, u32), (field_width, field_height): (u32, u32), square_size: u32) -> state::State {
    let distance_x = (field_width as i32/2 - x as i32).abs() as u32;
    let distance_y = (field_height as i32/2 - y as i32).abs() as u32;
    if distance_x < square_size && distance_y < square_size {state::State::Alive} else {state::State::Dead}
}

pub fn field_iteration(
    mut field_query: Query<&mut field::Field>,
    mut cells_query: Query<&mut cell::Cell>,
    mut field_timer: ResMut<timer::FieldTimer>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    // Tick the timer
    field_timer.timer.tick(time.delta());
    if !field_timer.timer.finished() {
        return;
    }

    if let Ok(mut field) = field_query.get_single_mut() {
        let mut new_field_cells = field.cells.clone();
        for mut cell in cells_query.iter_mut() {
            // Copying the current state - we will modify it
            let current_state = cell.state.clone();
            // Calculate number of neighbors
            let neighbors_number = field.neighbors_number(&cell.position);

            match current_state {
                state::State::Dead => {
                    if settings.rule.birth.contains(&neighbors_number) {
                        birth(&mut cell, &mut new_field_cells, settings.clone());
                    }
                }
                state::State::Alive => {
                    if !settings.rule.survive.contains(&neighbors_number) {
                        apply_damage(&mut cell, &mut new_field_cells);
                    }
                }
            }
        }

        field.cells = new_field_cells;
    }
}

fn apply_damage(cell: &mut cell::Cell, field_cells: &mut Vec<Vec<cell::Cell>>) {
    cell.health -= 1;
    if cell.health > 0 {
        return;
    }

    cell.state = state::State::Dead;
    field_cells[cell.position.0 as usize][cell.position.1 as usize].state = state::State::Dead;
    cell.health = 0;
}

fn birth(cell: &mut cell::Cell, field_cells: &mut Vec<Vec<cell::Cell>>, settings: Settings) {
    cell.state = state::State::Alive;
    field_cells[cell.position.0 as usize][cell.position.1 as usize].state = state::State::Alive;
    cell.health = settings.rule.health;
}