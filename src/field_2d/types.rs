use bevy::{prelude::Component, sprite::Sprite, render::color::Color};

#[derive(Default, Copy, Clone)]
pub enum State {
    #[default]
    Dead,
    Alive,
}

#[derive(Default, Copy, Clone, Component)]
pub struct Cell {
    pub state: State,
    pub position: (u32, u32),
}

impl Cell {
    pub fn make_alive(&mut self, sprite: &mut Sprite) {
        self.state = State::Alive;
        sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), 1.0);
    }

    pub fn make_dead(&mut self, sprite: &mut Sprite) {
        self.state = State::Dead;
        sprite.color = Color::rgba(sprite.color.r(), sprite.color.g(), sprite.color.b(), 0.0);
    }
}

#[derive(Component)]
pub struct Field {
    pub cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn neighbors_number(&self, position: &(u32, u32)) -> u8 {
        let (x, y) = (position.0 as usize, position.1 as usize);
        let mut neighbors_number = 0u8;

        let increment_closure = |state: State| -> u8 {
            match state {
                State::Alive => 1u8,
                State::Dead => 0u8,
            }
        };

        // Adjacent elements:
        if x > 0 {
            neighbors_number += increment_closure(self.cells[x-1][y].state);
        }
        if y > 0 {
            neighbors_number += increment_closure(self.cells[x][y-1].state);
        }
        if x < self.cells.len()-1 {
            neighbors_number += increment_closure(self.cells[x+1][y].state);
        }
        if y < self.cells[x].len()-1 {
            neighbors_number += increment_closure(self.cells[x][y+1].state);
        }

        // Diagonal elements:
        if x > 0 && y > 0 {
            neighbors_number += increment_closure(self.cells[x-1][y-1].state);
        }
        if x > 0 && y < self.cells[x-1].len()-1 {
            neighbors_number += increment_closure(self.cells[x-1][y+1].state);
        }
        if x < self.cells.len()-1 && y > 0 {
            neighbors_number += increment_closure(self.cells[x+1][y-1].state);
        }
        if x < self.cells.len()-1 && y < self.cells[x+1].len()-1 {
            neighbors_number += increment_closure(self.cells[x+1][y+1].state);
        }

        neighbors_number
    }
}
