use crate::prelude::*;

pub fn coords_to_index(ctx: &Context, pos: &Point2<f32>) -> usize {
    let (window_w, window_h) = graphics::drawable_size(ctx);
    println!("window {}x{}", window_w, window_h);

    let cell_w = window_w / SCREEN_WIDTH as f32;
    let cell_h = window_h / SCREEN_HEIGHT as f32;

    let x = (pos.x / cell_w).floor() as i32;
    let y = (pos.y / cell_h).floor() as i32;
    println!("cell: {},{}", x, y);
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn cell_to_coords(ctx: &Context, i: &usize) -> Point2<f32> {
    let (window_w, window_h) = graphics::drawable_size(ctx);
    let cell_w = window_w as i32 / SCREEN_WIDTH;
    let cell_h = window_h as i32 / SCREEN_HEIGHT;
    let x = *i as i32 % SCREEN_WIDTH;
    let y = *i as i32 / SCREEN_HEIGHT;

    let p_x = ((cell_w * x) + (cell_w / 2)) as f32;
    let p_y = ((cell_h * y) + (cell_h / 2)) as f32;

    println!("cell_w {} x {}", cell_w, x);
    println!("cell_h {} y {}", cell_h, y);

    Point2 { x: p_x, y: p_y }
}