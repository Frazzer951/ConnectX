mod agent;
mod board;
mod ui;

use agent::{player_turn, Agent};
use board::{Board, GameState, Pieces};
use macroquad::prelude::*;

const DEBUG: bool = true;
const LEFT_BUFFER: f32 = 250.0;
const WINDOW_WIDTH: f32 = 225.0;

const MAX_ROW: usize = 500;
const MAX_COL: usize = 500;

fn window_conf() -> Conf {
    Conf {
        window_title: "Connect X".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rows: usize = 6;
    let mut cols: usize = 7;
    let mut x_val: usize = 4;
    let mut player_one = Agent::Player;
    let mut player_two = Agent::Player;

    let mut board = Board::new(rows, cols);
    let mut running: bool = false;
    let mut current_turn: bool = false;
    let mut gamestate = GameState::OnGoing;

    let mut selected_move: Option<usize> = None;

    board.place(0, Pieces::P1);
    board.place(0, Pieces::P2);

    loop {
        let width: f32 = screen_width();
        let width_adj: f32 = width - LEFT_BUFFER;
        let height: f32 = screen_height();
        let square_size = (width_adj / cols as f32).min(height / rows as f32);

        let max_x = rows.min(cols);

        board.verify(rows, cols, x_val, LEFT_BUFFER, square_size);

        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            let mut settings_height = 45.0;

            egui::Window::new("Settings")
                .default_size([WINDOW_WIDTH, 1.0])
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .resizable(false)
                .enabled(!running)
                .show(egui_ctx, |ui| {
                    let size = ui.available_size();
                    settings_height = size[1] + 45.0;

                    if DEBUG {
                        ui.label(format!("Width: {width}"));
                        ui.label(format!("Width Adj: {width_adj}"));
                        ui.label(format!("Height: {height}"));
                        ui.label(format!("Square Size: {square_size}"));

                        ui.label(format!("Max X: {max_x}"));

                        ui.label(format!("Window Size: {size:?}"));
                        ui.separator();
                    }

                    ui::number_drag(ui, &mut rows, "Rows:", 1..=MAX_ROW);
                    ui::number_drag(ui, &mut cols, "Cols:", 1..=MAX_COL);
                    ui::number_drag(ui, &mut x_val, "X Val:", 1..=max_x);

                    ui::agent_selector(ui, "Player 1", &mut player_one);
                    ui::agent_selector(ui, "Player 2", &mut player_two);

                    ui.separator();

                    ui.centered_and_justified(|ui| {
                        if ui.button("Start").clicked() {
                            running = true;
                            current_turn = false;
                            board.reset();
                        }
                    });
                });

            egui::Window::new("Running")
                .default_size([WINDOW_WIDTH, 1.0])
                .anchor(egui::Align2::LEFT_TOP, [0.0, settings_height])
                .resizable(false)
                .show(egui_ctx, |ui| {
                    if DEBUG {
                        ui.label(format!("Chosen Move: {selected_move:?}"));

                        ui.separator();
                    }

                    if current_turn {
                        ui.label("Current Turn: Player 2");
                    } else {
                        ui.label("Current Turn: Player 1");
                    }

                    ui.label(format!("GameState: {gamestate:?}"));

                    ui.separator();

                    ui.centered_and_justified(|ui| {
                        if ui
                            .add_enabled(running, egui::Button::new("End Game"))
                            .clicked()
                        {
                            running = false;
                        }
                    });
                });
        });

        board.draw();

        if running {
            if !current_turn {
                // Player 1
                let chosen_move = match player_one {
                    Agent::Player => player_turn(&board, current_turn),
                };
                if let Some(col) = chosen_move {
                    selected_move = chosen_move;

                    if board.place(col, Pieces::P1) {
                        current_turn = !current_turn;
                    }
                };
            } else {
                // Player 2
                let chosen_move = match player_two {
                    Agent::Player => player_turn(&board, current_turn),
                };
                if let Some(col) = chosen_move {
                    selected_move = chosen_move;

                    if board.place(col, Pieces::P2) {
                        current_turn = !current_turn;
                    }
                };
            };

            gamestate = board.game_state();

            match gamestate {
                GameState::Tie | GameState::P1Win | GameState::P2Win => running = false,
                GameState::OnGoing => {}
            }
        }

        egui_macroquad::draw();

        next_frame().await
    }
}
