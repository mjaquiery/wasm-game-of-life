//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use wasm_game_of_life::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn tick() {
    let mut universe = Universe::new(Some(5), Some(5), None);
    // Build a line
    universe.set_cells(vec![
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ]);

    println!("{}", universe);
    // Run a tick
    universe.tick();
    println!("{}", universe);

    // Check the state of the universe
    assert_eq!(
        universe.get_cells(),
        vec![
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
        ]
    );
}

#[wasm_bindgen_test]
fn test_wrapping() {
    let mut universe = Universe::new(Some(5), Some(5), Some(0.0));
    // Blinker at the edge of the universe
    universe.set_cells(vec![
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 1, 1,
    ]);
    println!("{}", universe);
    universe.tick();
    println!("{}", universe);
    assert_eq!(
        universe.get_cells(),
        vec![
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
        ]);
}
