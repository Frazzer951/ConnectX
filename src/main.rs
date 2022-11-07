use macroquad::prelude::*;

use agent::{Agent, compute_turn};
use board::{Board, GameState};

mod agent;
mod board;
mod ui;

const DEBUG: bool = true;

// Game Constants
const LEFT_BUFFER: f32 = 250.0;
const WINDOW_WIDTH: f32 = 225.0;
const MAX_ROW: usize = 500;
const MAX_COL: usize = 500;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Turn {
    Player1,
    Player2,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Connect X".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Game Settings
    let mut rows: usize = 6;
    let mut cols: usize = 7;
    let mut x_val: usize = 4;
    let mut player_one = Agent::Player;
    let mut player_two = Agent::Player;

    // Game Variables
    let mut board = Board::new(rows, cols);
    let mut current_turn: Turn = Turn::Player1;
    let mut running: bool = false;
    let mut gamestate = GameState::OnGoing;
    let mut sleep_time: f64 = 0.0;
    let mut time_counter: f64 = get_time();

    // Debug Info
    let mut selected_move: usize = 0;

    loop {
        // Calculate Square Size for rendering
        let width: f32 = screen_width();
        let width_adj: f32 = width - LEFT_BUFFER;
        let height: f32 = screen_height();
        let square_size = (width_adj / cols as f32).min(height / rows as f32);

        // Calculate limit for the maximum X value
        let max_x = rows.min(cols);

        // Resize the board if needed
        board.verify(rows, cols, x_val);

        clear_background(WHITE);

        // EGUI
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
                    ui.add(egui::Slider::new(&mut sleep_time, 0.0..=5.0).text("Sleep"));

                    ui::agent_selector(ui, "Player 1", &mut player_one);
                    ui::agent_selector(ui, "Player 2", &mut player_two);

                    ui.separator();

                    ui.centered_and_justified(|ui| {
                        if ui.button("Start").clicked() {
                            running = true;
                            current_turn = Turn::Player1;
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

                    match current_turn {
                        Turn::Player1 => ui.label("Current Turn: Player 1"),
                        Turn::Player2 => ui.label("Current Turn: Player 2"),
                    };

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

        board.draw(square_size);

        // Calculate turns
        if running {
            if get_time() - time_counter >= sleep_time {
                let start_turn = current_turn;
                let cur_agent = match current_turn {
                    Turn::Player1 => &player_one,
                    Turn::Player2 => &player_two,
                };
                if let Some(col) =
                compute_turn(&mut current_turn, cur_agent, &mut board, square_size)
                {
                    selected_move = col;
                }

                if current_turn != start_turn {
                    time_counter = get_time();
                }
            }

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
