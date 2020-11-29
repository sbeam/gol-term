mod universe;

pub mod prelude {
    pub use ggez::event;
    pub use ggez::graphics;
    pub use ggez::{Context, GameResult};
    pub use ggez::mint::Point2;
    pub use crate::universe::*;
    pub const SCREEN_WIDTH: i32 = 10;
    pub const SCREEN_HEIGHT: i32 = 10;
    pub const FRAME_DURATION: f32 = 75.0;
}

use prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use ggez::input;

// Here we're defining how many quickly we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
const UPDATES_PER_SECOND: f32 = 8.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[derive(Debug)]
enum GameMode {
    Setup,
    // Playing,
    // Paused,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Cell {
    alive: bool,
    age: i8,
}

#[derive(Debug)]
struct State {
    mode: GameMode,
    cells: Vec<bool>,
    deltas: HashMap<usize, Cell>,
    last_update: Instant,
    mouse_down: bool,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Setup,
            cells: vec![false; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            last_update: Instant::now(),
            mouse_down: false,
            deltas: HashMap::with_capacity((SCREEN_WIDTH * SCREEN_HEIGHT) as usize),
        }
    }

    fn render_deltas(&mut self, ctx: &mut Context) {
        if !self.deltas.is_empty() {
            println!("there are {} cells growing or dying!", self.deltas.len());
        }
    }

    fn render_cells(&mut self, ctx: &mut Context) -> GameResult {
        let living: Vec<(usize, &bool)> = self.cells
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value)
            .collect::<Vec<_>>();

        for (index, _) in living {
            render_cell(ctx, &index, true).unwrap()
        }
        Ok(())
    }

    fn setup(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        if self.mouse_down {
            let pos = input::mouse::position(ctx);
            let i = coords_to_index(ctx, &pos);
            println!("click {:?} -> that's cell #{}", pos, i);
            if self.cells[i] {
                self.cells[i] = false;
            } else {
                self.cells[i] = true;
            }
            // self.render_deltas(ctx);
            self.mouse_down = false;
        }
        self.render_cells(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

impl event::EventHandler for State {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.mode {
            GameMode::Setup => self.setup(ctx),
            // GameMode::Playing => self.play(bterm),
            // GameMode::Paused => self.pause(bterm),
            _ => Ok(())
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            self.last_update = Instant::now();
        }
        // a GameResult is an Option? what if we had an error?
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
    }
}

fn render_cell(ctx: &mut Context, cell: &usize, alive: bool) -> GameResult {
    // println!("cell at {:?} is {}", cell, if alive { "1" } else { "0"});
    let coords = cell_to_coords(ctx, cell);
    // println!("circle goes at {:?}", coords);
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Point2 { x: 0.0, y: 0.0 },
        10.0,
        1.0,
        if alive { graphics::WHITE } else { graphics::BLACK },
    )?;
    graphics::draw(ctx, &circle, (coords,))
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;

    // TODO: this changes the window size but the x/y coords stay at 800x600, which fcks up the y-spacing
    // graphics::set_drawable_size(ctx, 800f32, 800f32).expect("Could not size the window!");
    let state = &mut State::new();
    event::run(ctx, event_loop, state)
}
