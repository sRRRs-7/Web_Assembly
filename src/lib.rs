mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::fmt::Display;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen]
pub struct Creature {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Creature {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|x| {
                if x % 2 == 0 || x % 5 == 0 {
                    Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Self { width, height, cells }
    }

    fn get_index(&self, row: u32, col:u32) -> usize {
        (row * self.width + col) as usize
    }

    fn alive_count_around(&self, row: u32, col: u32) -> u8  {
        let mut count = 0;
        for d_row in [self.height - 1, 0, 1].clone() {
            for d_col in [self.width - 1, 0, 1].clone() {
                if d_row == 0 && d_col == 0 {
                    continue;
                }
                let row_idx = (row + d_row) % self.height;
                let col_idx = (col + d_col) % self.width;

                let idx = self.get_index(row_idx, col_idx);
                count += self.cells[idx] as u8
            }
        };
        count
    }

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let live_count = self.alive_count_around(row, col);
                let index = self.get_index(row, col);
                let cell = cells[index];

                let next_cell = match (cell, live_count) {
                    (Cell::Alive, cnt) if cnt < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, cnt) if cnt > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _)  => otherwise,
                };

                cells[index] = next_cell;
            }
        }
        self.cells = cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {
                    '◦'
                } else {
                    '●'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use crate::Creature;

    #[test]
    fn test1() {
        let creature = Creature::new();
        assert_eq!(creature.width, 64);
    }
}
