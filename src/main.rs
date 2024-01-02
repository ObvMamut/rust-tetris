use oorandom::Rand32;
use ggez::{event, graphics, input::keyboard::{KeyCode, KeyInput}, Context, GameResult, GameError, glam::*, glam};
use std::collections::VecDeque;
use std::env;
use std::path::PathBuf;
use ggez::graphics::Canvas;
use crate::GameState::NotStarted;




mod data {
    const GRID_SIZE: (i16, i16) = (22,32); // 22*12 for tetris screen | 10*22 for title | 32*10 for additional info
    const GRID_CELL_SIZE: (i16, i16) = (64, 64); // 1 grid = 64 px * 64 px

    pub(crate) const SCREEN_SIZE: (f32, f32) = (
        GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
        GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );


    pub(crate) const DESIRED_FPS: u32 = 8;}

#[derive(PartialEq)]
enum GameState {
    Started,
    Ended,
    NotStarted
}

#[derive(PartialEq)]
struct Board {
    board: [[u32;12];22],
    rng: Rand32,
    last_block: Vec<Vec<u32>>
}

#[derive(PartialEq)]
struct Game {
    state : GameState,
    board : Board,
    score : u32,
}

impl Board {
    pub fn new() -> Self {

        let mut init_board: [[u32;12];22] = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1],
            [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 1],
            [1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1]

        ];

        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");

        let mut rng = Rand32::new(u64::from_ne_bytes(seed));


        Board {
            board : init_board,
            rng : rng,
            last_block : vec![]
        }

    }


    fn draw_rect(&self, canvas: &mut graphics::Canvas, pos: (f32, f32), color: [f32;4]) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect([pos.0*64f32, pos.1*64f32, 64f32, 64f32].into())
                .color(color)
        );
    }

    fn can_new_piece(&mut self) -> bool {
        return true
    }

    fn spawn_new_piece(&mut self) -> Vec<Vec<i32>> {
        let mut pos: Vec<Vec<i32>> = vec![];

        return pos
    }

    fn move_piece(&mut self) {
        if Board::can_new_piece(self) {
            let mut pieces_coords: Vec<Vec<i32>> = Board::spawn_new_piece(self);

        }
    }


    fn display_board(&mut self, canvas: &mut graphics::Canvas) {
        let mut coords: (f32, f32) = (404 as f32, 404 as f32);
        let mut color = [0.0, 0.0, 0.0, 1.0];
        for row in 0..22 {
            for index in 0..12 {
                //println!("{:?}", self.board[row][index]);
                if self.board[row][index] == 2 {
                    // light blue
                    color = [0.0, 1.0, 0.871, 1.0];
                } else if self.board[row][index] == 3 {
                    // green
                    color = [0.0, 1.0, 0.0, 1.0];
                } else if self.board[row][index] == 4 {
                    // dark blue
                    color = [0.0, 0.0, 1.0, 1.0];
                } else if self.board[row][index] == 5 {
                    // magenta
                    color = [0.686, 0.0, 1.0, 1.0];
                } else if self.board[row][index] == 6 {
                    // orange
                    color = [1.0, 0.635, 0.0, 1.0];
                } else if self.board[row][index] == 7 {
                    // red
                    color = [1.0, 0.0, 0.0, 1.0];
                } else if self.board[row][index] == 8 {
                    // yellow
                    color = [1.0, 0.965, 0.0, 1.0];
                } else {
                    color = [0.0, 0.0, 0.0, 0.0];
                }


                coords = (index as f32, row as f32 + 10.0);


                Board::draw_rect(self, canvas, coords, color)
                }
            }

        }
    }



impl Game {
    fn new() -> Self {

        Game {
            state : GameState::NotStarted,
            board : Board::new(),
            score : 0
        }
    }



}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::G) {
            println!("GAME STARTED")
        }

        while ctx.time.check_update_time(data::DESIRED_FPS) {
            if self.state == GameState::Started {

            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.349, 0.345, 0.639, 1.0]));



        // Display board

        let board = graphics::Image::from_path(ctx, "/board.png")?;

        let bo = glam::Vec2::new(0.0, 640.0);
        canvas.draw(&board, graphics::DrawParam::new().dest(bo));


        // Display title

        let title = graphics::Image::from_path(ctx, "/title.png")?;

        let tit = glam::Vec2::new(512.0, 64.0);
        canvas.draw(&title, graphics::DrawParam::new().dest(tit));

        //self.board.draw_rect(&mut canvas, (1.0, 10.0));
        self.board.display_board(&mut canvas);


        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        PathBuf::from("./assets")
    };

    let (ctx, events_loop) = ggez::ContextBuilder::new("tetris", "ObvMamut").add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(data::SCREEN_SIZE.0, data::SCREEN_SIZE.1))
        .build()?;

    let state = Game::new();
    event::run(ctx, events_loop, state)
}