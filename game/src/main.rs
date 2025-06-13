use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Filled,
}

struct Board {
    grid: [[Cell; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    fn draw(&self, tetromino: &Tetromino, x: isize, y: isize) {
        print!("\x1B[2J\x1B[1;1H"); // 画面クリア
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if tetromino.occupies(x, y, col, row) {
                    print!("[]");
                } else {
                    match self.grid[row][col] {
                        Cell::Empty => print!(" ."),
                        Cell::Filled => print!("[]"),
                    }
                }
            }
            println!();
        }
    }

    fn check_collision(&self, tetromino: &Tetromino, x: isize, y: isize) -> bool {
        for (dy, row) in tetromino.shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    let nx = x + dx as isize;
                    let ny = y + dy as isize;
                    if nx < 0 || nx >= WIDTH as isize || ny < 0 || ny >= HEIGHT as isize {
                        return true;
                    }
                    if self.grid[ny as usize][nx as usize] == Cell::Filled {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn fix_tetromino(&mut self, tetromino: &Tetromino, x: isize, y: isize) {
        for (dy, row) in tetromino.shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    let nx = x + dx as isize;
                    let ny = y + dy as isize;
                    if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                        self.grid[ny as usize][nx as usize] = Cell::Filled;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Tetromino {
    shape: [[u8; 4]; 4],
}

impl Tetromino {
    fn patterns() -> Vec<[[u8; 4]; 4]> {
        vec![
            // I
            [
                [0,0,0,0],
                [1,1,1,1],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // O
            [
                [0,1,1,0],
                [0,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // S
            [
                [0,1,1,0],
                [1,1,0,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // Z
            [
                [1,1,0,0],
                [0,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // J
            [
                [1,0,0,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // L
            [
                [0,0,1,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
            // T
            [
                [0,1,0,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0],
            ],
        ]
    }

    fn random() -> Self {
        let patterns = Tetromino::patterns();
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..patterns.len());
        Tetromino { shape: patterns[idx] }
    }

    fn occupies(&self, x: isize, y: isize, col: usize, row: usize) -> bool {
        let rel_x = col as isize - x;
        let rel_y = row as isize - y;
        if rel_x >= 0 && rel_x < 4 && rel_y >= 0 && rel_y < 4 {
            self.shape[rel_y as usize][rel_x as usize] == 1
        } else {
            false
        }
    }

    // 90度右回転
    fn rotate_right(&self) -> Self {
        let mut new_shape = [[0u8; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_shape[x][3 - y] = self.shape[y][x];
            }
        }
        Tetromino { shape: new_shape }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut board = Board::new();
    let mut x: isize = (WIDTH / 2 - 2) as isize;
    let mut y: isize = 0;
    let mut tetromino = Tetromino::random();
    let mut last_tick = Instant::now();

    loop {
        board.draw(&tetromino, x, y);

        // 入力処理
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Left => {
                        if !board.check_collision(&tetromino, x - 1, y) {
                            x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if !board.check_collision(&tetromino, x + 1, y) {
                            x += 1;
                        }
                    }
                    KeyCode::Down => {
                        if !board.check_collision(&tetromino, x, y + 1) {
                            y += 1;
                        }
                    }
                    KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('\n') => {
                        let rotated = tetromino.rotate_right();
                        if !board.check_collision(&rotated, x, y) {
                            tetromino = rotated;
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // 自動落下
        if last_tick.elapsed() >= Duration::from_millis(400) {
            if board.check_collision(&tetromino, x, y + 1) {
                board.fix_tetromino(&tetromino, x, y);
                x = (WIDTH / 2 - 2) as isize;
                y = 0;
                tetromino = Tetromino::random();
            } else {
                y += 1;
            }
            last_tick = Instant::now();
        }
    }
    disable_raw_mode().unwrap();
}
