use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{
    board::{Board, GameState},
    Turn,
};

#[derive(PartialEq, Debug)]
pub enum Agent {
    Player,
    Random,
    AlphaBeta,
}

pub fn compute_turn(
    current_turn: &mut Turn,
    agent: &Agent,
    board: &mut Board,
    piece_size: f32,
) -> Option<usize> {
    let chosen_move = match agent {
        Agent::Player => player_turn(board, current_turn, piece_size),
        Agent::Random => random_turn(board),
        Agent::AlphaBeta => alpha_beta_turn(board, current_turn),
    };
    if let Some(col) = chosen_move {
        if board.place(col, current_turn) {
            *current_turn = current_turn.next();
        }
        return chosen_move;
    };
    None
}

pub fn player_turn(board: &Board, turn: &Turn, piece_size: f32) -> Option<usize> {
    let mouse_psn = mouse_position();

    let col = board.mouse_hover(piece_size, mouse_psn, turn);

    if col.is_some() && is_mouse_button_pressed(MouseButton::Left) {
        col
    } else {
        None
    }
}

pub fn random_turn(board: &Board) -> Option<usize> {
    let moves = board.moves();

    if !moves.is_empty() {
        moves.choose().cloned()
    } else {
        None
    }
}

pub fn alpha_beta_turn(board: &Board, turn: &Turn) -> Option<usize> {
    let (col, _) = minimax(board.clone(), 5, i32::MIN, i32::MAX, true, turn);
    col
}

fn minimax(
    board: Board,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    turn: &Turn,
) -> (Option<usize>, i32) {
    if depth == 0 {
        return (None, board.score_position(turn));
    }

    let board_state = board.game_state();

    if board_state != GameState::OnGoing {
        if board_state == GameState::P1Win {
            return match turn {
                Turn::Player1 => (None, i32::MAX),
                Turn::Player2 => (None, i32::MIN),
            };
        } else if board_state == GameState::P2Win {
            return match turn {
                Turn::Player1 => (None, i32::MIN),
                Turn::Player2 => (None, i32::MAX),
            };
        } else {
            return (None, 0);
        }
    }

    let valid_locations = board.moves();

    if valid_locations.is_empty() {
        // We should never get here but its just in case
        return (None, 0);
    }

    if maximizing_player {
        let mut value = i32::MIN;
        let mut column = *valid_locations.choose().unwrap();
        for col in valid_locations {
            let b_copy = board.result(col, turn);
            let (_, new_score) = minimax(b_copy, depth - 1, alpha, beta, false, turn);
            if new_score > value {
                value = new_score;
                column = col;
            }
            alpha = alpha.max(value);
            if alpha >= beta {
                break;
            }
        }
        (Some(column), value)
    } else {
        let mut value = i32::MAX;
        let mut column = *valid_locations.choose().unwrap();
        for col in valid_locations {
            let b_copy = board.result(col, &turn.next());
            let (_, new_score) = minimax(b_copy, depth - 1, alpha, beta, true, turn);
            if new_score < value {
                value = new_score;
                column = col;
            }
            beta = beta.min(value);
            if alpha >= beta {
                break;
            }
        }
        (Some(column), value)
    }
}
