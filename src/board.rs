use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pieces {
    P1,
    P2,
    Empty,
}

#[derive(Debug)]
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

pub struct Board {
    rows: usize,
    cols: usize,
    x_to_win: usize,
    board: Vec<Vec<Pieces>>,
    left_buffer: f32,
    piece_size: f32,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Board {
            rows,
            cols,
            x_to_win: 0,
            board: vec![vec![Pieces::Empty; cols]; rows],
            left_buffer: 0.0,
            piece_size: 0.0,
        }
    }

    pub fn verify(
        &mut self,
        rows: usize,
        cols: usize,
        x_to_win: usize,
        left_buffer: f32,
        piece_size: f32,
    ) {
        if self.rows != rows || self.cols != cols {
            self.rows = rows;
            self.cols = cols;
            self.board = vec![vec![Pieces::Empty; cols]; rows]
        }
        if self.x_to_win != x_to_win {
            self.x_to_win = x_to_win;
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

    pub fn place(&mut self, col: usize, piece: Pieces) -> bool {
        for row in (0..self.rows).rev() {
            if self.board[row][col] == Pieces::Empty {
                self.board[row][col] = piece;
                return true;
            }
        }
        false
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

    pub fn game_state(&self) -> GameState {
        let mut full = true;

        // Check if top row is filled
        for col in 0..self.cols {
            if self.board[0][col] == Pieces::Empty {
                full = false;
                break;
            }
        }

        // Check all diagonal and cardinal direction
        for i in 0..self.rows {
            for j in 0..self.cols {
                let cur_piece = self.board[i][j];
                if cur_piece == Pieces::Empty {
                    continue;
                }

                // Check right
                if j + self.x_to_win <= self.cols {
                    let mut count = 1;
                    for col in j + 1..self.cols {
                        if self.board[i][col] == cur_piece {
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
                        if self.board[row][j] == cur_piece {
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
                        if self.board[i - offset][j + offset] == cur_piece {
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
                        if self.board[i + offset][j + offset] == cur_piece {
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
}
