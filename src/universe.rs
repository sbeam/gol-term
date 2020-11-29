use crate::prelude::*;

const NUM_CELLS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub fn coords_to_index(pos: &Point2<f32>) -> usize {
    ((pos.y * SCREEN_WIDTH as f32) + pos.x) as usize
}
