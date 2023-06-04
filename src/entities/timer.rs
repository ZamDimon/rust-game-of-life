use bevy::{ecs::system::Resource, prelude::{Timer, TimerMode}};
use std::time::Duration;

#[derive(Resource)]
pub struct FieldTimer {
    pub timer: Timer,
}

impl FieldTimer {
    pub fn new(frequency: Duration) -> FieldTimer {
        FieldTimer {
            timer: Timer::new(frequency, TimerMode::Repeating),
        }
    }
}
