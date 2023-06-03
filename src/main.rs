#[macro_use]
extern crate lazy_static;
extern crate itertools;

use bevy::{prelude::*, window::PrimaryWindow};
use rand;
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config cannot be loaded!");
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn init_field(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let block_size = CONFIG.clone().field_settings.block_size;

    let field_width_in_cells: i32 = (window.width() / block_size) as i32;
    let field_height_in_cells: i32 = (window.height() / block_size) as i32;
    for (x, y) in itertools::iproduct!(0..field_width_in_cells, 0..field_height_in_cells) {
        if rand::random() {
            continue;
        }
        let (x_pos, y_pos) = (
            x as f32 * block_size + block_size / 2f32,
            y as f32 * block_size + block_size / 2f32,
        );
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(block_size, block_size)),
                ..default()
            },
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            ..default()
        });
    }
}

fn main() {
    let cfg = CONFIG.clone();

    App::new()
        .insert_resource(ClearColor(cfg.default_window_settings.clear_color))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        cfg.default_window_settings.width,
                        cfg.default_window_settings.height,
                    )
                        .into(),
                    title: cfg.default_window_settings.name,
                    ..default()
                }),
                ..default()
            }),
        )
        .add_startup_system(spawn_camera)
        .add_startup_system(init_field)
        .run();
}
