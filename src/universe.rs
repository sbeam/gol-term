use crate::prelude::*;

pub fn coords_to_index(pos: &Point2<f32>) -> usize {
    let x = (pos.x / CELL_W as f32).floor() as i32;
    let y = (pos.y / CELL_H as f32).floor() as i32;
    // println!("cell: {},{}", x, y);
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn cell_to_coords(i: &usize) -> Point2<f32> {
    let x = *i as i32 % SCREEN_WIDTH;
    let y = *i as i32 / SCREEN_HEIGHT;

    let p_x = ((CELL_W * x) + (CELL_W / 2)) as f32;
    let p_y = ((CELL_H * y) + (CELL_H / 2)) as f32;

    // println!("cell_w {} x {}", cell_w, x);
    // println!("cell_h {} y {}", cell_h, y);

    Point2 { x: p_x, y: p_y }
}

pub fn generation(cells: &mut Vec<bool>) -> Vec<bool> {
    let r = SCREEN_WIDTH as usize;
    let mut new_cells = cells.clone();

    for (i, &alive) in cells.iter().enumerate() {
        if i > r && i < r * (SCREEN_HEIGHT as usize - 1) {
            let adjacents: [bool; 8] = if i % r == 0 {
                // left edge col, wraps "up"
                [
                    cells[i+1],
                    cells[i-r],
                    cells[i+r],
                    cells[i-r+1],
                    cells[i+r+1],
                    cells[i-r-1],
                    cells[i-1],
                    cells[i+(2 * r)-1],
                ]
            } else if i % r == r - 1 {
                // right side col, wraps "down"
                [
                    cells[i-1], 
                    cells[i+1], 
                    cells[i-r], 
                    cells[i+r], 
                    cells[i-r-1], 
                    cells[if i > 2*r { i-(2 * r) +1 } else { 0 }],
                    cells[i+r-1], 
                    cells[i-r+1], 
                ]
            } else {
                [
                    cells[i-1],
                    cells[i+1],
                    cells[i-r],
                    cells[i+r],
                    cells[i-r-1],
                    cells[i-r+1],
                    cells[i+r-1],
                    cells[i+r+1],
                ]
            };
            let neighbors = adjacents
                .iter()
                .filter(|&&value| value)
                .count();
            println!("Cell {} has {} neighbors", i, neighbors);

            new_cells[i] = if alive {
                !(neighbors < 2 || neighbors > 3)
            } else {
                neighbors == 3
            }
        }
    }
    new_cells
}

