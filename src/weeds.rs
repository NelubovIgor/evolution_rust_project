use ggez::graphics::{Rect};
use rand::Rng;

pub struct Weed {
    pub rect: Rect,
}

impl Weed {
    pub fn make_weed() -> Weed {
        let x = rand::thread_rng().gen_range(0..500) as f32;
        let y = rand::thread_rng().gen_range(0..500) as f32;
        let weed = Weed {
            rect: Rect::new(x, y, 3.0, 3.0),
        };
        weed
    }
}
