//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use wasm_snake::snake::{FieldSize, Snake, *};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn initialize_snake() {
    // initializing snake
    let mut s: Snake = Snake::spawn(FieldSize::new(10, 10));
    assert_eq!(s.size, 1);
}

#[wasm_bindgen_test]
fn grow_snake() {
    // initializing snake
    let mut s: Snake = Snake::spawn(FieldSize::new(50, 50));

    // growing snake
    for _x in 0..9 {
        s.grow_snake();
    }

    assert_eq!(s.size, 10);
}

#[wasm_bindgen]
fn snake_movement() {
    // initializing snake
    let mut s: Snake = Snake::spawn(FieldSize::new(50, 50));

    // growing snake upto 5 units
    for _x in 0..4 {
        s.grow_snake();
    }

    s.move_snake(Direction::UP);
}
