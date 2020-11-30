use crate::prelude::*;

pub fn coords_to_index(ctx: &Context, pos: &Point2<f32>) -> usize {
    let x = (pos.x / CELL_W as f32).floor() as i32;
    let y = (pos.y / CELL_H as f32).floor() as i32;
    // println!("cell: {},{}", x, y);
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn cell_to_coords(ctx: &Context, i: &usize) -> Point2<f32> {
    let x = *i as i32 % SCREEN_WIDTH;
    let y = *i as i32 / SCREEN_HEIGHT;

    let p_x = ((CELL_W * x) + (CELL_W / 2)) as f32;
    let p_y = ((CELL_H * y) + (CELL_H / 2)) as f32;

    // println!("cell_w {} x {}", cell_w, x);
    // println!("cell_h {} y {}", cell_h, y);

    Point2 { x: p_x, y: p_y }
}
