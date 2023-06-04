use bevy::prelude::Component;
use crate::entities::{cell::Cell, state::State};

#[derive(Component)]
pub struct Field {
    pub cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn neighbors_number(&self, position: &(u32, u32)) -> u32 {
        let (x, y) = (position.0 as usize, position.1 as usize);
        let mut neighbors_number = 0u32;

        let increment_closure = |state: State| -> u32 {
            match state {
                State::Alive => 1u32,
                State::Dead => 0u32,
            }
        };

        // Adjacent elements:
        if x > 0 {
            neighbors_number += increment_closure(self.cells[x - 1][y].state);
        }
        if y > 0 {
            neighbors_number += increment_closure(self.cells[x][y - 1].state);
        }
        if x < self.cells.len() - 1 {
            neighbors_number += increment_closure(self.cells[x + 1][y].state);
        }
        if y < self.cells[x].len() - 1 {
            neighbors_number += increment_closure(self.cells[x][y + 1].state);
        }

        // Diagonal elements:
        if x > 0 && y > 0 {
            neighbors_number += increment_closure(self.cells[x - 1][y - 1].state);
        }
        if x > 0 && y < self.cells[x - 1].len() - 1 {
            neighbors_number += increment_closure(self.cells[x - 1][y + 1].state);
        }
        if x < self.cells.len() - 1 && y > 0 {
            neighbors_number += increment_closure(self.cells[x + 1][y - 1].state);
        }
        if x < self.cells.len() - 1 && y < self.cells[x + 1].len() - 1 {
            neighbors_number += increment_closure(self.cells[x + 1][y + 1].state);
        }

        neighbors_number
    }
}
