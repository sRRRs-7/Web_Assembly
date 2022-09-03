
pub mod macros;
pub mod utils;
pub mod toggle;
pub mod mover;
pub mod performance;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use js_sys;
use macros::*;

use std::{fmt::Display, time};
use wasm_timer::Delay;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    pub fn new(rng1: u32, rng2: u32) -> Self {
        utils::set_panic_hook();

        let width = 35;
        let height = 35;

        let cells = (0..width * height)
            .map(|x| {
                if x % rng1 == 0 || x % rng2 == 0 {
                    Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Self { width, height, cells }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_index(&self, row: u32, col:u32) -> usize {
        (row * self.width + col) as usize
    }

    // pub fn alive_count_around(&self, row: u32, col: u32) -> u8  {
    //     let mut count = 0;
    //     for d_row in [self.height - 1, 0, 1].clone() {
    //         for d_col in [self.width - 1, 0, 1].clone() {
    //             if d_row == 0 && d_col == 0 {
    //                 continue;
    //             }
    //             let row_idx = (row + d_row) % self.height;
    //             let col_idx = (col + d_col) % self.width;

    //             let idx = self.get_index(row_idx, col_idx);
    //             count += self.cells[idx] as u8
    //         }
    //     };
    //     count
    // }

    pub fn alive_count_around(&self, row: u32, col: u32) -> u8  {
        let mut count = 0;

        let top = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let bottom = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let left = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };

        let right = if col == self.width - 1 {
            0
        } else {
            col + 1
        };

        let tl = self.get_index(top, left);
        count += self.cells[tl] as u8;

        let t = self.get_index(top, col);
        count += self.cells[t] as u8;

        let tr = self.get_index(top, right);
        count += self.cells[tr] as u8;

        let r = self.get_index(row, right);
        count += self.cells[r] as u8;

        let br = self.get_index(bottom, right);
        count += self.cells[br] as u8;

        let b = self.get_index(bottom, col);
        count += self.cells[b] as u8;

        let bl = self.get_index(bottom, left);
        count += self.cells[bl] as u8;

        let l = self.get_index(row, left);
        count += self.cells[l] as u8;

        count
    }

    pub fn tick(&mut self) {
        // let _timer = performance::Timer::new("Life Game");

        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let live_count = self.alive_count_around(row, col);
                let index = self.get_index(row, col);
                let cell = cells[index];

                let cell_state = match (cell, live_count) {
                    (Cell::Alive, cnt) if cnt < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, cnt) if cnt > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _)  => otherwise,
                };
                cells[index] = cell_state;
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

#[wasm_bindgen]
pub fn get_random(min: f64, max: f64) -> f64 {
    let float: f64 = js_sys::Math::random() * (max - min) + min;
    let rng = js_sys::Math::ceil(float);
    rng
}

#[wasm_bindgen]
pub async fn sleep_millis(numbers: u16) -> js_sys::Promise {
  let millis: u64 = u64::from(numbers);
  Delay::new(time::Duration::from_millis(millis)).await.unwrap();

  let promise = js_sys::Promise::resolve(&numbers.into());
  return promise;
}

#[cfg(test)]
mod tests {
    use crate::Creature;

    #[test]
    fn test1() {
        let creature = Creature::new(2, 7);
        assert_eq!(creature.width, 65);
    }

    #[test]
    fn divide() {
        let mut creature = Creature::new(1, 10);
        creature.set_width(3);
        creature.set_height(3);
        let calc = creature.width() * creature.height() / 2;
        assert_eq!(calc, 4);
    }
}
