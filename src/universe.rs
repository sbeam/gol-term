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
    let w = SCREEN_WIDTH as isize;
    let h = SCREEN_HEIGHT as isize;
    let mut new_cells = cells.clone();

    for (i, &alive) in cells.iter().enumerate() {
        let i = i as isize;
        let up = if i < w {
            (w * (h - 1)) as isize
        } else {
            -(w as isize)
        };
        let down = if i >= (w * (h - 1)) {
            -((w * (h - 1)) as isize)
        } else {
            w as isize
        };
        let right = if i % w == w - 1 {
            -(w - 1) as isize
        } else {
            1isize
        };
        let left = if i % w == 0 {
            (w - 1) as isize
        } else {
            -1isize
        };

        let adjacents: [bool; 8] = [
            cells[(i + left + up) as usize],
            cells[(i + up) as usize],
            cells[(i + right + up) as usize],
            cells[(i + left) as usize],
            cells[(i + right) as usize],
            cells[(i + left + down) as usize],
            cells[(i + down) as usize],
            cells[(i + right + down) as usize],
        ];
        let neighbors = adjacents.iter().filter(|&&value| value).count();
        // println!("Cell {} has {} neighbors", i, neighbors);

        new_cells[i as usize] = if alive {
            !(neighbors < 2 || neighbors > 3)
        } else {
            neighbors == 3
        }
    }
    new_cells
}
