use crate::{board::Board, Turn};
use macroquad::{prelude::*, rand::ChooseRandom};

#[derive(PartialEq, Debug)]
pub enum Agent {
    Player,
    Random,
}

pub fn compute_turn(current_turn: &mut Turn, agent: &Agent, board: &mut Board) -> Option<usize> {
    let chosen_move = match agent {
        Agent::Player => player_turn(board, current_turn),
        Agent::Random => random_turn(board),
    };
    if let Some(col) = chosen_move {
        if board.place(col, current_turn) {
            current_turn.next();
        }
        return chosen_move;
    };
    None
}

pub fn player_turn(board: &Board, turn: &Turn) -> Option<usize> {
    let mouse_psn = mouse_position();

    let col = board.mouse_hover(mouse_psn, turn);

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
