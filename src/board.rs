use macroquad::prelude::*;
use ndarray::prelude::*;

use crate::{Turn, LEFT_BUFFER};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pieces {
    P1,
    P2,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    OnGoing,
    P1Win,
    P2Win,
    Tie,
}

const P1_COLOR: Color = Color::new(0.90, 0.16, 0.22, 1.00);
const P1_COLOR_TRANS: Color = Color::new(0.90, 0.16, 0.22, 0.50);
const P2_COLOR: Color = Color::new(0.99, 0.98, 0.00, 1.00);
const P2_COLOR_TRANS: Color = Color::new(0.99, 0.98, 0.00, 0.50);

#[derive(Debug, Clone)]
pub struct Board {
    rows: usize,
    cols: usize,
    x_to_win: usize,
    board: Array1<Pieces>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Board {
            rows,
            cols,
            x_to_win: 0,
            board: Array1::from_elem(rows * cols, Pieces::Empty),
        }
    }

    pub fn verify(&mut self, rows: usize, cols: usize, x_to_win: usize) {
        if self.rows != rows || self.cols != cols {
            self.rows = rows;
            self.cols = cols;
            self.board = Array1::from_elem(rows * cols, Pieces::Empty);
        }
        if self.x_to_win != x_to_win {
            self.x_to_win = x_to_win;
        }
    }

    pub fn reset(&mut self) {
        self.board = Array1::from_elem(self.rows * self.cols, Pieces::Empty)
    }

    pub fn piece_at(&self, row: usize, col: usize) -> Pieces {
        self.board[[row * self.cols + col]]
    }

    pub fn set_piece(&mut self, row: usize, col: usize, piece: Pieces) {
        self.board[[row * self.cols + col]] = piece
    }

    pub fn place(&mut self, col: usize, turn: &Turn) -> bool {
        for row in (0..self.rows).rev() {
            if self.piece_at(row, col) == Pieces::Empty {
                self.set_piece(
                    row,
                    col,
                    match turn {
                        Turn::Player1 => Pieces::P1,
                        Turn::Player2 => Pieces::P2,
                    },
                );

                return true;
            }
        }
        false
    }

    pub fn mouse_hover(&self, piece_size: f32, psn: (f32, f32), turn: &Turn) -> Option<usize> {
        let x = psn.0 - LEFT_BUFFER;
        if x < 0.0 || x > self.cols as f32 * piece_size {
            return None;
        }

        let col = (x / piece_size) as usize;

        let x_pos = LEFT_BUFFER + col as f32 * piece_size;
        let height = self.rows as f32 * piece_size;
        let color = match turn {
            Turn::Player1 => P1_COLOR_TRANS,
            Turn::Player2 => P2_COLOR_TRANS,
        };

        draw_rectangle(x_pos, 0.0, piece_size, height, color);

        Some(col)
    }

    pub fn draw(&self, piece_size: f32) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut x = LEFT_BUFFER + j as f32 * piece_size;
                let mut y = i as f32 * piece_size;
                draw_rectangle(x, y, piece_size - 1.0, piece_size - 1.0, GRAY);

                x += piece_size / 2.0;
                y += piece_size / 2.0;
                match self.piece_at(i, j) {
                    Pieces::P1 => draw_circle(x, y, piece_size / 2.5, P1_COLOR),
                    Pieces::P2 => draw_circle(x, y, piece_size / 2.5, P2_COLOR),
                    Pieces::Empty => draw_circle(x, y, piece_size / 2.5, WHITE),
                }
            }
        }
    }

    pub fn moves(&self) -> Vec<usize> {
        let mut moves = vec![];

        for col in 0..self.cols {
            if self.piece_at(0, col) == Pieces::Empty {
                moves.push(col);
            }
        }

        moves
    }

    pub fn result(&self, col: usize, turn: &Turn) -> Self {
        let mut new_board = self.clone();
        new_board.place(col, turn);
        new_board
    }

    pub fn game_state(&self) -> GameState {
        let mut full = true;

        // Check if top row is filled
        for col in 0..self.cols {
            if self.piece_at(0, col) == Pieces::Empty {
                full = false;
                break;
            }
        }

        // Check all diagonal and cardinal direction
        for i in 0..self.rows {
            for j in 0..self.cols {
                let cur_piece = self.piece_at(i, j);
                if cur_piece == Pieces::Empty {
                    continue;
                }

                // Check right
                if j + self.x_to_win <= self.cols {
                    let mut count = 1;
                    for col in j + 1..self.cols {
                        if self.piece_at(i, col) == cur_piece {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= self.x_to_win {
                        return if cur_piece == Pieces::P1 {
                            GameState::P1Win
                        } else {
                            GameState::P2Win
                        };
                    }
                }

                // Check down
                if i + self.x_to_win <= self.rows {
                    let mut count = 1;
                    for row in i + 1..self.rows {
                        if self.piece_at(row, j) == cur_piece {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= self.x_to_win {
                        return if cur_piece == Pieces::P1 {
                            GameState::P1Win
                        } else {
                            GameState::P2Win
                        };
                    }
                }

                // Check up diagonal
                if i as i32 - self.x_to_win as i32 >= -1 && j + self.x_to_win <= self.cols {
                    let mut count = 1;
                    for offset in 1..self.x_to_win {
                        if self.piece_at(i - offset, j + offset) == cur_piece {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= self.x_to_win {
                        return if cur_piece == Pieces::P1 {
                            GameState::P1Win
                        } else {
                            GameState::P2Win
                        };
                    }
                }
                // Check down diagonal
                if i + self.x_to_win <= self.rows && j + self.x_to_win <= self.cols {
                    let mut count = 1;
                    for offset in 1..self.x_to_win {
                        if self.piece_at(i + offset, j + offset) == cur_piece {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= self.x_to_win {
                        return if cur_piece == Pieces::P1 {
                            GameState::P1Win
                        } else {
                            GameState::P2Win
                        };
                    }
                }
            }
        }

        // If board is full and no one has won, then its a tie
        if full {
            return GameState::Tie;
        }

        // Game is still going
        GameState::OnGoing
    }

    /// Code modified from https://github.com/KeithGalli/Connect4-Python
    pub fn score_position(&self, turn: &Turn) -> i32 {
        let mut score: i32 = 0;

        let cur_piece = match turn {
            Turn::Player1 => Pieces::P1,
            Turn::Player2 => Pieces::P2,
        };

        let x_m1 = self.x_to_win - 1;

        // Score center column
        let center_count = self
            .board
            .slice(s![(self.cols / 2)..;self.cols])
            .iter()
            .filter(|&p| *p == cur_piece)
            .count() as i32;
        score += center_count * 3;

        // Score Horizontal
        for r in 0..self.rows {
            let r_num = r * self.cols;
            let row_array = self.board.slice(s![r_num..(r_num + self.cols)]);
            for c in 0..(self.cols - x_m1) {
                let window = row_array.slice(s![c..c + self.x_to_win]);
                score += evaluate_window(&window, cur_piece, self.x_to_win);
            }
        }

        // Score Vertical
        for c in 0..self.cols {
            let col_array = self.board.slice(s![c..;self.cols]);
            for r in 0..(self.rows - x_m1) {
                let window = col_array.slice(s![r..r + self.x_to_win]);
                score += evaluate_window(&window, cur_piece, self.x_to_win);
            }
        }

        // Score Diagonals
        // Negative Diagonal
        for r in 0..(self.rows - x_m1) {
            for c in 0..(self.cols - x_m1) {
                let ip = r * self.cols + c;
                let fp = (r + x_m1) * self.cols + (c + x_m1);
                let ss = self.cols + 1;
                let window = self.board.slice(s![ip..=fp;ss]);

                score += evaluate_window(&window, cur_piece, self.x_to_win);
            }
        }

        // Positive Diagonal
        for r in 0..(self.rows - x_m1) {
            for c in 0..(self.cols - x_m1) {
                let ip = r * self.cols + (c + x_m1);
                let fp = (r + x_m1) * self.cols + c;
                let ss = self.cols - 1;
                let window = self.board.slice(s![ip..=fp;ss]);

                score += evaluate_window(&window, cur_piece, self.x_to_win);
            }
        }

        score
    }
}

/// Code modified from https://github.com/KeithGalli/Connect4-Python
fn evaluate_window(window: &ArrayView1<Pieces>, piece: Pieces, x: usize) -> i32 {
    let mut score = 0;

    let opp_piece = match piece {
        Pieces::P1 => Pieces::P2,
        Pieces::P2 => Pieces::P1,
        Pieces::Empty => Pieces::Empty,
    };

    let piece_count = window.iter().filter(|&p| *p == piece).count();
    let opp_count = window.iter().filter(|&p| *p == opp_piece).count();

    if opp_count == 0 {
        if piece_count == x {
            score += 100;
        } else if piece_count >= x / 2 {
            score += 5;
        } else if piece_count > 1 {
            score += 2;
        }
    } else if piece_count == 0 && opp_count >= (0.75 * x as f32) as usize {
        score -= 4;
    }

    score
}
