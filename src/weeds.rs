use ggez::graphics::{Rect};
use rand::Rng;

use crate::World;
use crate::constants;

pub struct Weed {
    pub rect: Rect,
    pos: u32,
    color: char,
}

impl Weed {
    pub fn make_weed(world: &mut Vec<World>) -> Weed {
        let x = rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32;
        let y = rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32;
        let possition = (y * constants::HEIGHT + x) as u32;
        let cell = world.get_mut(possition as usize).unwrap();
        cell.color = 'g';
        let weed = Weed {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            pos: possition,
            color: 'g',
        };
        weed
    }
}
