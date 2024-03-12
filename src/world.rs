use crate::constants;
use ggez::graphics::Rect;
use nalgebra::Point2;
use rand::Rng;
// use crate::agents::Agent; 


// let agent: Agent;

// agent = crate::agent;

// let mut agent: Option<> = None;
// let mut world = Vec::new();

// enum CellType {
//     Weed,
//     Agent,
//     Path,
// }

#[derive(Clone, Debug)]
pub struct World {
    pub pos: Point2<f32>,
    pub rect: Rect,
    pub energy: f32,
    // pub celltype: CellType,
    pub type_cell: char,
}


impl World {
    pub fn make_cells(x: Option<f32>, y: Option<f32>) -> World {
        let x = match x {
            Some(x) => x,
            None => rand::thread_rng().gen_range(0..constants::WIDTH as u32) as f32,
        };
        let y = match y {
            Some(y) => y,
            None => rand::thread_rng().gen_range(0..constants::HEIGHT as u32) as f32,
        };
        let _pos: Point2<f32> = Point2::new(x, y);
        let cell: World = World {
            pos: _pos,
            rect: Rect::new(x, y, constants::SIZE_CELL, constants::SIZE_CELL),
            energy: 0.0,
            // celltype: Path,
            type_cell: 'b',
        };    
        cell
    }
}
