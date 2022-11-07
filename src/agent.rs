use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{board::Board, Turn};

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

pub fn alpha_beta_turn(_board: &Board, _turn: &Turn) -> Option<usize> {
    None
}
