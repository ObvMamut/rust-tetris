#[allow(unused)]

use ggez::{event, graphics, input::keyboard::{KeyCode}, Context, GameResult, GameError, glam::*, glam};
use std::env;
use std::path::PathBuf;
use rand::Rng;
use ggez::input::keyboard::KeyInput;

mod data {
    const GRID_SIZE: (i16, i16) = (22,32); // 22*12 for tetris screen | 10*22 for title | 32*10 for additional info
    const GRID_CELL_SIZE: (i16, i16) = (64, 64); // 1 grid = 64 px * 64 px

    pub(crate) const SCREEN_SIZE: (f32, f32) = (
        GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
        GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );


    pub(crate) const DESIRED_FPS: u32 = 60;
}

#[derive(PartialEq)]
enum GameState {
    Started,
    Ended,
    NotStarted
}

#[derive(PartialEq)]
struct Board {
    board : [[i32;12];22],
    last_block : Vec<Vec<i32>>,
    last_block_id : i32,
    fall_updater : i32
}

#[derive(PartialEq)]
struct Game {
    state : GameState,
    board : Board,
    score : i32,
}

impl Board {
    pub fn new() -> Self {

        let mut init_board: [[i32;12];22] = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ,1]

        ];




        Board {
            board : init_board,
            last_block : vec![vec![]],
            last_block_id : 0,
            fall_updater : 0
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

    fn piece_can_move(&mut self, mut coords: Vec<Vec<i32>>) -> bool {


        let mut coords_to_check: Vec<Vec<i32>> = vec![];

        for coord in coords {
            if self.last_block.contains(&coord.clone()) {

            } else {
                coords_to_check.push(coord.clone());
            }
        }

        //println!("{:?}", coords_to_check);


        for single_coord in coords_to_check {
            if self.board[single_coord[0] as usize][single_coord[1] as usize] != 0 {
                return false
            }
        }


        return true
    }

    fn fall_block(&mut self) {
        let mut bin_checker = 0;
        let block_coords = self.last_block.clone();
        let mut new_block_coords: Vec<Vec<i32>> = vec![];


        if self.fall_updater == 60 {
            // second has passed
            bin_checker = 1;
            self.fall_updater = 0;
        } else {
            self.fall_updater += 1;
        }

        if bin_checker == 1 {

            for coord in &block_coords {
                let new_coord = vec![(coord[0] + 1), coord[1]];
                new_block_coords.push(new_coord);
            }



            if Board::piece_can_move(self, new_block_coords.clone()) == true {

                // remove the old blocks

                for coord in block_coords.clone() {
                    self.board[coord[0] as usize][coord[1] as usize] = 0;
                }

                for coord in &new_block_coords {
                    self.board[coord[0] as usize][coord[1] as usize] = self.last_block_id;
                }

                self.last_block = new_block_coords.clone();


            } else {
                self.last_block = vec![vec![]];
                self.last_block_id = 0;
            }

        }


    }

    fn spawn_new_piece(&mut self) {

        let mut rng = rand::thread_rng();
        let rand_int =  rng.gen_range(2..9);
        let mut coords_of_piece: Vec<Vec<i32>>;


        match rand_int {
            2 => {
                // light blue / horizontal 1x4
                coords_of_piece = vec![vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7]];
            }
            3 => {
                // green / z (inversed)
                coords_of_piece = vec![vec![2, 5], vec![2, 6], vec![1, 6], vec![1, 7]];

            }
            4 =>  {
                // dark blue / l (inversed) horizontal
                coords_of_piece = vec![vec![1, 4], vec![2, 4], vec![2, 5], vec![2, 6]];

            }
            5 => {
                // magenta / inversed hat
                coords_of_piece = vec![vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 5]];

            }
            6 => {
                // orange / l horizontal
                coords_of_piece = vec![vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 5]];

            }
            7 => {
                // red
                coords_of_piece = vec![vec![1, 6], vec![2, 4], vec![2, 5], vec![2, 6]];

            }
            8 => {
                // yellow 2x2
                coords_of_piece = vec![vec![1, 5], vec![1, 6], vec![2, 5], vec![2, 6]];

            }

            _ => {
                coords_of_piece = vec![vec![]];
            }

        }

        if coords_of_piece != vec![vec![]] {
            if Board::piece_can_move(self, coords_of_piece.clone()) {
                for coord in &coords_of_piece {
                    self.board[coord[0] as usize][coord[1] as usize] = rand_int;
                }

                self.last_block = coords_of_piece;
                self.last_block_id = rand_int;
            }



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

    fn move_piece(&mut self, move_type: &str) {
        let mut coords: Vec<Vec<i32>> = vec![vec![]];

        match move_type {
            "A" => {
                // move left
                coords = Board::move_left(self);
            },
            "W" => {
                // rotate

                coords = Board::rotate(self);
            },
            "S" => {
                // down
                coords = Board::get_drop_coords(self);
            },
            "D" => {
                // right
                coords = Board::move_right(self);
            },
            _ => {
                coords = vec![vec![]];
            }
        }

        if coords != vec![vec![]] {

            if Board::piece_can_move(self, coords.clone()) == true {
                for coord in self.last_block.clone() {
                    self.board[coord[0] as usize][coord[1] as usize] = 0;
                }

                for coord in &coords {
                    self.board[coord[0] as usize][coord[1] as usize] = self.last_block_id;
                }

                self.last_block = coords.clone();
            }
        }


    }

    fn get_drop_coords(&mut self) -> Vec<Vec<i32>> {
        let mut crds = vec![];

        for x in 1..23 {
            for crd in &self.last_block {
                let mut crd_to_push = vec![crd[0] + x, crd[1]];
                crds.push(crd_to_push);
            }

            if Board::piece_can_move(self, crds.clone()) == false {
                crds = vec![];
                for crd in &self.last_block {
                    let mut crd_to_push = vec![crd[0] + x - 1, crd[1]];
                    crds.push(crd_to_push);
                }

                return crds
            }

            crds = vec![];
        }

        return crds
    }

    fn coords_to_coords(&mut self, crds: Vec<i32>) -> Vec<i32> {
        let mut coord = vec![];
        let mut x: i32;
        let mut y: i32;

        x = (21 - crds[0] as i32);
        y = (crds[1] as i32);

        coord.push(x);
        coord.push(y);



        return coord
    }



    fn rotate(&mut self) -> Vec<Vec<i32>> {
        let mut coords : Vec<Vec<i32>> = vec![];
        let mut rot_coords : Vec<Vec<i32>> = vec![];
        let mut rot_bo_coords : Vec<Vec<i32>> = vec![];

        for c1 in self.last_block.clone() {
            let mut coord = Board::coords_to_coords(self, c1);
            coords.push(coord);
        }

        let mut pivot = &coords[0];

        let xc = pivot[0];
        let yc = pivot[1];

        for c2 in coords.clone() {
            let mut x1 = ((c2[0] - xc) * 0) - ((c2[1] - yc) * -1) + xc;
            let mut y1 = ((c2[0] - xc) * -1) + ((c2[1] - yc) * 0) + yc;

            let mut crds = vec![x1, y1];
            rot_coords.push(crds);
        }

        for c3 in rot_coords.clone() {
            let mut coord = Board::coords_to_coords(self, c3);
            rot_bo_coords.push(coord);
        }

        println!("{:?}", rot_bo_coords);

        return rot_bo_coords.clone()
    }



    fn move_left(&mut self) -> Vec<Vec<i32>> {
        let mut new_coords: Vec<Vec<i32>> = vec![];

        for coord in &self.last_block {
            let new_coord = vec![(coord[0]), coord[1] - 1];
            new_coords.push(new_coord);
        }

        return new_coords;
    }
    fn move_right(&mut self) -> Vec<Vec<i32>> {
        let mut new_coords: Vec<Vec<i32>> = vec![];

        for coord in &self.last_block {
            let new_coord = vec![(coord[0]), coord[1] + 1];
            new_coords.push(new_coord);
        }

        return new_coords;
    }
    fn move_down(&mut self) -> Vec<Vec<i32>> {
        let mut new_coords: Vec<Vec<i32>> = vec![];

        for coord in &self.last_block {
            let new_coord = vec![(coord[0] + 1), coord[1]];
            new_coords.push(new_coord);
        }

        return new_coords;
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
        while ctx.time.check_update_time(data::DESIRED_FPS) {

            if self.state == GameState::Started {
                println!("{:?}", self.board.last_block);

                if self.board.last_block != vec![vec![]] {
                    println!("{:?}", self.board.fall_updater);
                    self.board.fall_block();
                } else {
                    self.board.spawn_new_piece();
                }


                println!("{:?}", ctx.keyboard.pressed_keys());


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

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, ) -> GameResult {
        match input.keycode {
            Some(KeyCode::W) => {
                self.board.move_piece("W");
            }
            Some(KeyCode::A) => {
                self.board.move_piece("A");
            }
            Some(KeyCode::D) => {
                self.board.move_piece("D");
            }
            Some(KeyCode::S) => {
                self.board.move_piece("S");
            }
            Some(KeyCode::G) => {
                if self.state == GameState::NotStarted {
                    self.state = GameState::Started;
                    self.board.spawn_new_piece();
                }
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }
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