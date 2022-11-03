use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pieces {
    P1,
    P2,
    Empty,
}

pub struct Board {
    rows: usize,
    cols: usize,
    board: Vec<Vec<Pieces>>,
    left_buffer: f32,
    piece_size: f32,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Board {
            rows,
            cols,
            board: vec![vec![Pieces::Empty; cols]; rows],
            left_buffer: 0.0,
            piece_size: 0.0,
        }
    }

    pub fn verify(&mut self, rows: usize, cols: usize, left_buffer: f32, piece_size: f32) {
        if self.rows != rows || self.cols != cols {
            self.rows = rows;
            self.cols = cols;
            self.board = vec![vec![Pieces::Empty; cols]; rows]
        }
        if self.left_buffer != left_buffer {
            self.left_buffer = left_buffer;
        }
        if self.piece_size != piece_size {
            self.piece_size = piece_size;
        }
    }

    pub fn reset(&mut self) {
        self.board = vec![vec![Pieces::Empty; self.cols]; self.rows]
    }

    pub fn place(&mut self, col: usize, piece: Pieces) {
        for row in (0..self.rows).rev() {
            if self.board[row][col] == Pieces::Empty {
                self.board[row][col] = piece;
                return;
            }
        }
    }

    pub fn draw(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut x = self.left_buffer + j as f32 * self.piece_size;
                let mut y = i as f32 * self.piece_size;
                draw_rectangle(x, y, self.piece_size - 1.0, self.piece_size - 1.0, GRAY);

                x += self.piece_size / 2.0;
                y += self.piece_size / 2.0;
                match self.board[i][j] {
                    Pieces::P1 => draw_circle(x, y, self.piece_size / 2.5, RED),
                    Pieces::P2 => draw_circle(x, y, self.piece_size / 2.5, YELLOW),
                    Pieces::Empty => draw_circle(x, y, self.piece_size / 2.5, WHITE),
                }
            }
        }
    }
}
