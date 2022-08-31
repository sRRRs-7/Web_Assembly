//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_lib::Creature;


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn tick_test() {
    let mut input = specific_input();
    let expect = expected_result();
    input.tick();
    assert_ne!(&input.get_cells(), &expect.get_cells());
}


#[cfg(test)]
fn specific_input() -> Creature {
    let mut creature = Creature::new(1, 10);
    creature.set_width(10);
    creature.set_height(10);
    creature.set_cells(&[(1,2), (3,4), (4,3), (3,1), (2,3)]);
    creature
}

#[cfg(test)]
fn expected_result() -> Creature {
    let mut creature = Creature::new(1, 10);
    creature.set_width(10);
    creature.set_height(10);
    creature.set_cells(&[(1,2), (3,4), (4,3), (1,3), (2,3)]);
    creature
}
