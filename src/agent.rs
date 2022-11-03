use crate::board::Board;
use macroquad::prelude::*;

#[derive(PartialEq, Debug)]
pub enum Agent {
    Player,
}

pub fn player_turn(board: &Board, player: bool) -> Option<usize> {
    let mouse_psn = mouse_position();

    let col = board.mouse_hover(mouse_psn, player);

    if col.is_some() && is_mouse_button_pressed(MouseButton::Left) {
        col
    } else {
        None
    }
}
