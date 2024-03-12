use ggez::graphics::Rect;
use rand::Rng;
use nalgebra::geometry::Point2;

// use rand::seq::SliceRandom; 

use crate::World;
use crate::constants;

#[derive(Copy, Clone)]
pub struct Weed {
    pub energy: f32,
    pub rect: Rect,
    pub pos: Point2<f32>,
    pub type_cell: char,
}

impl Weed {
    pub fn make_weed(world: &mut Vec<World>, x: Option<f32>, y: Option<f32>) -> Weed {
        let x: f32 = match x {
            Some(x) => x,
            None => rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32,
        };
        let y: f32 = match y {
            Some(y) => y,
            None => rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32,
        };
        let _pos: Point2<f32> = Point2::new(x, y);
        let _energy: f32 = 50.0;


        let cell: &mut World = world.iter_mut().find(|w| w.pos == _pos).expect("No cell found");
        // cell.celltype = Weed;
        cell.type_cell = 'g';
        cell.energy = _energy;

        let weed: Weed = Weed {
            energy: _energy,
            pos: _pos,
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            type_cell: 'g',
        };
        weed
    }

    // pub fn grow_weed(&self, world: &mut Vec<World>) -> Option<Weed> {
    //     let mut empty_side = Vec::new();
    //     let north = self.rect.x + 1.0;
    //     let south = self.rect.x - 1.0;
    //     let east = self.rect.y + 1.0;
    //     let west = self.rect.y - 1.0;
        
    //     if 0.0 <= north && north <= constants::WIDTH && world[(self.rect.y as f32 * constants::HEIGHT + north as f32) as usize].color == 'b' {
    //         empty_side.push((north, self.rect.y));
    //     }
    //     if 0.0 <= south && south <= constants::WIDTH && world[(self.rect.y as f32 * constants::HEIGHT + south as f32) as usize].color == 'b' {
    //         empty_side.push((south, self.rect.y));
    //     }
    //     if 0.0 <= east && east <= constants::WIDTH && world[(east as f32 * constants::HEIGHT + self.rect.x as f32) as usize].color == 'b' {
    //         empty_side.push((east, self.rect.x));
    //     }
    //     if 0.0 <= west && west <= constants::WIDTH && world[(west as f32 * constants::HEIGHT + self.rect.x as f32) as usize].color == 'b' {
    //         empty_side.push((west, self.rect.x));
    //     }
    //     if !empty_side.is_empty() {
    //         if let Some(&(x, y)) = empty_side.choose(&mut rand::thread_rng()) {
    //             Some(Weed::make_weed(world, Some(x), Some(y)))
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
        
    // }
}
