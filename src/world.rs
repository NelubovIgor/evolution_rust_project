use crate::constants;
use ggez::graphics::{Rect};
// use crate::agents::Agent;


// let agent: Agent;

// agent = crate::agent;

// let mut agent: Option<> = None;
// let mut world = Vec::new();

#[derive(Clone)]
pub struct World {
    pos: u32,
    pub x: u32,
    pub y: u32,
    pub rect: Rect,
    pub color: char,
}

impl World {
    pub fn make_cells() -> Vec<World> {
        let mut cells = Vec::new();
        for i in 0..(constants::WIDTH * constants::HEIGHT) as u32 {
            let x_pos = i % constants::HEIGHT as u32;
            let y_pos = i / constants::HEIGHT as u32;
            let cell = World {
                pos: i,
                x: x_pos,
                y: y_pos,
                rect: Rect::new(x_pos as f32, y_pos as f32, constants::SIZE_CELL, constants::SIZE_CELL),
                color: 'b',
            };
            cells.push(cell)
        }
        cells
    }
}
