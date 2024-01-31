use ggez::graphics::{Rect};
use rand::Rng;

use crate::constants;

pub struct Weed {
    pub rect: Rect,
}

impl Weed {
    pub fn make_weed() -> Weed {
        let x = rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32;
        let y = rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32;
        let weed = Weed {
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
        };
        weed
    }
}
