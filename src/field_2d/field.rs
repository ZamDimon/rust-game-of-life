use crate::field_2d::types;
use crate::settings::parser::Settings;
use bevy::{prelude::*, window::PrimaryWindow};
use rand;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_random_field);
        app.add_system(field_iteration);
    }
}

#[derive(Resource)]
pub struct FieldTimer {
    timer: Timer,
}

pub fn init_random_field(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    config: Res<Settings>,
) {
    // Some calculations regarding a field size
    let window = window_query.get_single().unwrap();
    let block_size = config.field_settings.block_size;
    let field_width_in_cells: usize = (window.width() / block_size) as usize;
    let field_height_in_cells: usize = (window.height() / block_size) as usize;

    // Get alive cell color
    let alive_color = config.field_settings.alive_cell_color;

    // Initializing a field entity
    let mut field = types::Field {
        cells: vec![vec![Default::default(); field_width_in_cells]; field_height_in_cells],
    };

    for (x, y) in itertools::iproduct!(0..field_width_in_cells, 0..field_height_in_cells) {
        let rand_seed = rand::random();

        let (x_pos, y_pos) = (
            x as f32 * block_size + block_size / 2f32,
            y as f32 * block_size + block_size / 2f32,
        );

        let new_cell = types::Cell {
            state: match rand_seed {
                false => types::State::Dead,
                true => types::State::Alive,
            },
            position: (x as u32, y as u32),
        };

        field.cells[x][y] = new_cell.clone();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    // if rand_seed is 0f, then color will be fully transparent
                    color: Color::rgba(
                        alive_color.r(),
                        alive_color.g(),
                        alive_color.b(),
                        rand_seed as u8 as f32,
                    ),
                    custom_size: Some(Vec2::new(block_size, block_size)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                ..default()
            },
            new_cell,
        ));
    }

    // Adding field entity
    commands.spawn(field);
    // Adding a timer
    commands.insert_resource(FieldTimer {
        timer: Timer::new(
            config.field_settings.updating_frequency,
            TimerMode::Repeating,
        ),
    });
}

pub fn field_iteration(
    mut field_query: Query<&mut types::Field>,
    mut cells_query: Query<(&mut types::Cell, &mut Sprite)>,
    time: Res<Time>,
    mut field_timer: ResMut<FieldTimer>,
) {
    // Tick the timer
    field_timer.timer.tick(time.delta());
    if !field_timer.timer.finished() {
        return;
    }

    if let Ok(mut field) = field_query.get_single_mut() {
        let mut new_field_cells = field.cells.clone();
        for (mut cell, mut sprite) in cells_query.iter_mut() {
            let cell: &mut types::Cell = &mut cell;
            let sprite: &mut Sprite = &mut sprite;
            let current_state = cell.state.clone();
            // Calculate number of neighbors
            let neighbors_number = field.neighbors_number(&cell.position);

            match current_state {
                types::State::Dead => {
                    if neighbors_number == 3 {
                        cell.make_alive(sprite);
                        new_field_cells[cell.position.0 as usize][cell.position.1 as usize].state =
                            types::State::Alive;
                    }
                }
                types::State::Alive => {
                    if neighbors_number != 2 && neighbors_number != 3 {
                        cell.make_dead(sprite);
                        new_field_cells[cell.position.0 as usize][cell.position.1 as usize].state =
                            types::State::Dead;
                    }
                }
            }
        }

        field.cells = new_field_cells;
    }
}
