use crate::entities::*;
use crate::settings::local::Settings;
use bevy::prelude::*;

#[derive(Default, Copy, Clone, Component)]
pub struct Cell {
    pub state: state::State,
    pub position: (u32, u32),
    pub health: u32,
}

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl CellBundle {
    pub fn cell(&self) -> Cell {
        self.cell
    }

    pub fn new(
        (x, y): (u32, u32),
        state: state::State,
        settings: Settings,
    ) -> CellBundle {
        let block_size = settings.field_settings.block_size;

        let (x_pos, y_pos) = (
            x as f32 * block_size + block_size / 2f32,
            y as f32 * block_size + block_size / 2f32,
        );

        let new_cell = cell::Cell {
            state: state,
            position: (x as u32, y as u32),
            health: settings.rule.health,
        };

        CellBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    // if rand_seed is 0f, then color will be fully transparent
                    color: settings.field_settings.cell_color_to,
                    custom_size: Some(Vec2::new(block_size, block_size)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                ..default()
            },
            cell: new_cell,
        }
    }
}