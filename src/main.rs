#[macro_use]
extern crate lazy_static;
extern crate itertools;

// Modules
mod settings;
mod game;
mod entities;

// Bevy imports
use bevy::{prelude::*, window::PrimaryWindow};
use crate::settings::local::CONFIG;
use crate::game::plugin::GameOfLifePlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(CONFIG.clone().window_plugin()));
    app.insert_resource(ClearColor(CONFIG.default_window_settings.clear_color));
    app.insert_resource(CONFIG.clone());
    app.add_startup_system(spawn_camera);
    app.add_plugin(GameOfLifePlugin);

    app.run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}