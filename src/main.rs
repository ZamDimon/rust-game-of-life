#[macro_use]
extern crate lazy_static;
extern crate itertools;

// Modules
mod settings;
mod field_2d;

// Bevy imports
use bevy::{prelude::*, window::PrimaryWindow};
use crate::settings::global::CONFIG;
use crate::field_2d::field::FieldPlugin;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn main() {
    let cfg = CONFIG.clone();

    App::new()
        .insert_resource(ClearColor(cfg.default_window_settings.clear_color))
        .insert_resource(CONFIG.clone())
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
        .add_plugin(FieldPlugin)
        .add_startup_system(spawn_camera)
        .run();
}
