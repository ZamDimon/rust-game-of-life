#[macro_use]
extern crate lazy_static;
use bevy::prelude::*;
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config cannot be loaded!");
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let cfg = CONFIG.clone();
    print!("{:?}", cfg.default_window_settings.clear_color);

    App::new().
        insert_resource(ClearColor(cfg.default_window_settings.clear_color))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                resolution: (cfg.default_window_settings.width, cfg.default_window_settings.height).into(),
                title: cfg.default_window_settings.name,
                ..default() 
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}
