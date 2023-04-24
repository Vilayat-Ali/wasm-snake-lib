#![allow(dead_code, unused)]

use std::fmt::{Display, Formatter};

use wasm_bindgen::{convert::FromWasmAbi, prelude::wasm_bindgen, JsObject};

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct FieldSize {
    pub rows: u64,
    pub cols: u64,
}

#[wasm_bindgen]
impl FieldSize {
    pub fn new(rows: u64, cols: u64) -> Self {
        Self { rows, cols }
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Coord {
    pub x: u64,
    pub y: u64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", &self.x, &self.y)
    }
}

#[wasm_bindgen]
impl Coord {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn centered(field_size: FieldSize) -> Self {
        // field_size = tuple(no_of_rows, no_of_columns)
        Self {
            x: f32::ceil((field_size.cols as f32) / 2_f32) as u64,
            y: f32::ceil((field_size.rows as f32) / 2_f32) as u64,
        }
    }
}

pub type SnakeBodyNode = Option<Box<Node>>;

#[derive(Clone)]
pub struct Node {
    data: Coord,
    next: SnakeBodyNode,
}

impl Node {
    pub fn new(data: Coord, next: SnakeBodyNode) -> Self {
        Self { data, next }
    }

    pub fn gen_snake_body_node(data: Coord, next: SnakeBodyNode) -> SnakeBodyNode {
        Some(Box::new(Self { data, next }))
    }
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Snake {
    head: SnakeBodyNode,
    pub size: u32,
    pub direction: Direction,
    pub speed: u32,
    pub field_size: FieldSize,
}

#[wasm_bindgen]
impl Snake {
    pub fn spawn(field_size: FieldSize) -> Self {
        Self {
            head: Some(Box::new(Node::new(
                Coord::centered(field_size.clone()),
                None,
            ))),
            size: 1,
            direction: Direction::UP,
            speed: 12,
            field_size,
        }
    }

    pub fn get_movement_data(&self) -> Vec<Coord> {
        let mut movement_list: Vec<Coord> = Vec::with_capacity(100);

        if *&self.size == 1 {
            movement_list.push(self.head.clone().unwrap().data)
        } else {
            let mut start: &SnakeBodyNode = &self.head;
            while start.unwrap().next.is_some() {
                match start {
                    Some(ref node) => {
                        // gathering info from data node
                        movement_list.push(node.clone().data);
                        // updating start inorder to continue traversal
                        start = &node.next;
                    }
                    None => {}
                }
            }
            // processing tail node
            // gathering info from data node
            movement_list.push(start.clone().unwrap().data);
        }

        movement_list
    }

    fn next_head_coord(&self) -> Coord {
        let mut new_coord: Coord = self.head.clone().unwrap().data;

        match &self.direction {
            Direction::UP => {
                // moving up means snake goes -1 in Y axis
                new_coord.y -= 1;
            }
            Direction::DOWN => {
                // moving up means snake goes +1 in Y axis
                new_coord.y += 1;
            }
            Direction::LEFT => {
                // moving up means snake goes -1 in X axis
                new_coord.x -= 1;
            }
            Direction::RIGHT => {
                // moving up means snake goes 1 in X axis
                new_coord.x += 1;
            }
        }

        new_coord
    }

    pub fn grow_snake(&mut self) {
        // snake grows by appending one node at the head of the list
        let mut new_head_coord: Coord = self.next_head_coord();
        let rest_of_snake_body: SnakeBodyNode = self.head.take();
        let new_snake_head: SnakeBodyNode =
            Node::gen_snake_body_node(new_head_coord, rest_of_snake_body);

        self.head = new_snake_head;
        self.size += 1;
    }

    fn reduce_snake_by_one(&mut self) {
        if self.size > 1 {
            let head: SnakeBodyNode = self.head.take();

            match head {
                Some(node_val) => {
                    self.head = node_val.next;
                    self.size -= 1;
                }
                None => {}
            }
        }
    }

    pub fn move_snake(&mut self, direction: Direction) -> bool {
        let mut next_head_coord: Coord = self.next_head_coord();

        if next_head_coord.x < u64::MIN
            || next_head_coord.x >= self.field_size.rows
            || next_head_coord.y < u64::MIN
            || next_head_coord.y >= self.field_size.cols
        {
            // snake head touched the boundary
            // game over condition
            return false;
        } else {
            // add a block at head
            self.grow_snake();
            // remove a block at tail
            self.reduce_snake_by_one();
        }
        return true;
    }
}
