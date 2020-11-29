mod universe;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::universe::*;
    pub const SCREEN_WIDTH: i32 = 90;
    pub const SCREEN_HEIGHT: i32 = 90;
    pub const FRAME_DURATION: f32 = 75.0;
}

use prelude::*;

#[derive(Debug)]
enum GameMode {
    Setup,
    // Playing,
    // Paused,
}

#[derive(Debug)]
struct Deltas {
    living: Vec<Point>,
    dying: Vec<Point>,
}

#[derive(Debug)]
struct State {
    mode: GameMode,
    cells: Vec<bool>,
    frame_time: f32,
    deltas: Deltas,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Setup,
            cells: vec![false; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            frame_time: FRAME_DURATION,
            deltas: Deltas {
                living: vec![],
                dying: vec![],
            }
        }
    }

    fn render_deltas(&mut self, bterm: &mut BTerm) {
        if self.deltas.living.is_empty()> {
            println!("there are {} cells being born!", self.deltas.living.len());
            for cell in &self.deltas.living {
                let i = coords_to_index(cell.x, cell.y);
                self.cells[i] = true;
                bterm.set_bg(cell.x, cell.y, RGB::named(WHITE));
            }
            self.deltas.living.clear();
        }
        if self.deltas.dying.is_empty() {
            println!("there are {} cells dying!", self.deltas.dying.len());
            for cell in &self.deltas.dying {
                let i = coords_to_index(cell.x, cell.y);
                self.cells[i] = false;
                bterm.set_bg(cell.x, cell.y, RGB::named(BLACK));
            }
            self.deltas.dying.clear();
        }
    }

    fn setup(&mut self, bterm: &mut BTerm) {
        if bterm.left_click { // AAAFACGGGK! true both on down and up!
            println!("click");
            let (x, y) = bterm.mouse_pos();
            let i = coords_to_index(x, y);
            if self.cells[i] {
                self.deltas.dying.push(Point::new(x, y));
            } else {
                self.deltas.living.push(Point::new(x, y));
            }
            self.render_deltas(bterm);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        match self.mode {
            GameMode::Setup => self.setup(bterm),
            // GameMode::Playing => self.play(bterm),
            // GameMode::Paused => self.pause(bterm),
        }
    }
}

fn main() -> BError {
    let bterm = BTermBuilder::simple80x50()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_title("GAME OF LIFE")
        .build()?;
    main_loop(bterm, State::new())
}
