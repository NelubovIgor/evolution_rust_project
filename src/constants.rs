use nalgebra::point;
use nalgebra::{OPoint, Const};
pub const WIDTH: f32 = 500.0;
pub const MAIN_WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 500.0;
pub const SIZE_CELL: f32 = 1.0;
// pub static points: Vec<OPoint<i32, Const<2>>> = [
//     point![-1, -1], 
//     point![-1, 0],
//     point![-1, 1],
//     point![0, -1],
//     point![0, 1],
//     point![1, -1],
//     point![1, 0],
//     point![1, 1],
// ].to_vec();

pub static POINTS: [OPoint<i32, Const<2>>; 8] = [
    point![-1, -1],
    point![-1, 0],
    point![-1, 1],
    point![0, -1],
    point![0, 1],
    point![1, -1],
    point![1, 0],
    point![1, 1],
];
