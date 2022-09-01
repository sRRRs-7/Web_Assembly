use crate::{ Creature, Cell, ceil};

use wasm_bindgen::prelude::*;
use crate::macros::*;

#[wasm_bindgen]
impl Creature {
    pub fn set_glider(&mut self, row: u32, col: u32) {
        let w: usize = self.width as usize;

        let index = self.get_index(row, col);
        self.cells[index].toggle();
        self.cells[index - w - 1].toggle();
        self.cells[index + 1].toggle();
        self.cells[index + w - 1].toggle();
        self.cells[index + w].toggle();
    }

    pub fn set_pulse(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        let w = (self.width) as usize;

        self.cells[index - w - 1].toggle();
        self.cells[index - 2 * w - 1].toggle();
        self.cells[index - w].toggle();
        self.cells[index - w + 1].toggle();
        self.cells[index - 2 * w + 1].toggle();
        self.cells[index - 1].toggle();
        self.cells[index + 1].toggle();
        self.cells[index + w - 1].toggle();
        self.cells[index + 2 * w - 1].toggle();
        self.cells[index + w].toggle();
        self.cells[index + w + 1].toggle();
        self.cells[index + 2 * w + 1].toggle();
    }
}