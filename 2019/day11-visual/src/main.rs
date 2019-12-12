use std::fs;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::conf;

use day11_visual::intcode::color::Color;
use std::time::{SystemTime, UNIX_EPOCH};

use day11_visual;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let grid: Vec<Vec<Color>> = day11_visual::solve(&input);
    
    let len = grid[0].len();
    let height = grid.len();
    
    let grid: Vec<Color> = grid.iter().flatten().map(|x| x.clone()).collect();

    let (mut ctx, mut event_loop) =
        ContextBuilder::new("game_name", "author_name")
            .window_mode(conf::WindowMode::default().dimensions((len * 10) as f32, (height * 20) as f32))
            .build()
            .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = IntCodeResult::new(&mut ctx, grid, len);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct IntCodeResult {
    index: usize,
    result: Vec<Color>,
    len: usize,
    last_update: u128
}

impl IntCodeResult {
    fn new(_ctx: &mut Context, result: Vec<Color>, len: usize) -> Self {
        Self {
            index: 0,
            result,
            len,
            last_update: 0
        }
    }
}

impl EventHandler for IntCodeResult {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        if now - self.last_update < 50 {
            return Ok(());
        }

        if self.index + 1 < self.result.len() {
            self.index += 1;
        } else {
            ggez::event::quit(_ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let (cell_w, cell_h) = (10, 20);

        let drawings: Vec<ggez::GameResult<graphics::Mesh>> = self.result[..=self.index]
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let (y, x) = (i / self.len, i % self.len);

                let color = match e {
                    Color::Black => graphics::BLACK,
                    Color::White => graphics::WHITE
                };

                graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32((x * cell_w) as i32, (y * cell_h) as i32, cell_w as i32, cell_h as i32),
                    color,
                )
            })
            .collect();

        for d in drawings {
            graphics::draw(ctx, &d.unwrap(), (na::Point2::new(0.0, 0.0),))?;
        }

        graphics::present(ctx)
    }
}