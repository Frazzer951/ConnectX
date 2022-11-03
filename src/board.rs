use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pieces {
    P1,
    P2,
    Empty,
}

const P1_COLOR: Color = Color::new(0.90, 0.16, 0.22, 1.00);
const P1_COLOR_TRANS: Color = Color::new(0.90, 0.16, 0.22, 0.50);
const P2_COLOR: Color = Color::new(0.99, 0.98, 0.00, 1.00);
const P2_COLOR_TRANS: Color = Color::new(0.99, 0.98, 0.00, 0.50);

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

    pub fn mouse_hover(&self, psn: (f32, f32), player: bool) -> Option<usize> {
        let x = psn.0 - self.left_buffer;
        if x < 0.0 || x > self.cols as f32 * self.piece_size {
            return None;
        }

        let col = (x / self.piece_size) as usize;

        let x_pos = self.left_buffer + col as f32 * self.piece_size;
        let height = self.rows as f32 * self.piece_size;
        let color = if player {
            P2_COLOR_TRANS
        } else {
            P1_COLOR_TRANS
        };

        draw_rectangle(x_pos, 0.0, self.piece_size, height, color);

        Some(col)
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
                    Pieces::P1 => draw_circle(x, y, self.piece_size / 2.5, P1_COLOR),
                    Pieces::P2 => draw_circle(x, y, self.piece_size / 2.5, P2_COLOR),
                    Pieces::Empty => draw_circle(x, y, self.piece_size / 2.5, WHITE),
                }
            }
        }
    }
}
