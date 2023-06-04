use crate::entities::*;
use crate::settings::local::Settings;
use bevy::prelude::*;

pub fn render_cells(mut cells_query: Query<(&cell::Cell, &mut Sprite)>, settings: Res<Settings>) {
    for (cell, mut sprite) in cells_query.iter_mut() {
        sprite.color = match cell.state {
            state::State::Alive => color_interpolate(
                &settings.field_settings.cell_color_from,
                &settings.field_settings.cell_color_to,
                0.5 + 0.5 * cell.health as f32 / settings.rule.health as f32,
            ),
            state::State::Dead => Color::rgba(
                settings.field_settings.cell_color_to.r(),
                settings.field_settings.cell_color_to.g(),
                settings.field_settings.cell_color_to.b(),
                0.0,
            ),
        }
    }
}

fn color_interpolate(from: &Color, to: &Color, ratio: f32) -> Color {
    return Color::rgb(
        from.r() + (to.r() - from.r()) * ratio,
        from.g() + (to.g() - from.g()) * ratio,
        from.b() + (to.b() - from.b()) * ratio,
    );
}
