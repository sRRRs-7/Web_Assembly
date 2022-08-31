use crate::{ Creature, Cell};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Creature {
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }
}

impl Creature {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells:&[(u32, u32)]) {
        for (row, col) in cells.iter().clone() {
            let index = self.get_index(*row, *col);
            self.cells[index] = Cell::Alive;
        }
    }
}
