mod board;

use board::{Board, Pieces};
use macroquad::prelude::*;

const DEBUG: bool = true;
const LEFT_BUFFER: f32 = 200.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Connect X".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn ui_number_drag(ui: &mut egui::Ui, val: &mut usize, text: &str) {
    ui.columns(2, |columns| {
        columns[0].label(text);
        columns[1].add(egui::DragValue::new(val));
    });
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rows: usize = 6;
    let mut cols: usize = 7;
    let mut x_val: usize = 4;

    let mut board = Board::new(rows, cols);

    board.place(0, Pieces::P1);
    board.place(0, Pieces::P1);
    board.place(0, Pieces::P1);
    board.place(1, Pieces::P2);

    loop {
        let width: f32 = screen_width();
        let width_adj: f32 = width - LEFT_BUFFER;
        let height: f32 = screen_height();
        let square_size = (width_adj / cols as f32).min(height / rows as f32);

        board.verify(rows, cols, LEFT_BUFFER, square_size);

        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            let mut settings_height = 45.0;
            egui::Window::new("Settings")
                .default_size([185.0, 1.0])
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .enabled(true)
                .show(egui_ctx, |ui| {
                    let size = ui.available_size();
                    settings_height = size[1] + 45.0;

                    if DEBUG {
                        ui.label(format!("Width: {width}"));
                        ui.label(format!("Width Adj: {width_adj}"));
                        ui.label(format!("Height: {height}"));
                        ui.label(format!("Square Size: {square_size}"));

                        ui.label(format!("Window Size: {size:?}"));
                        ui.add(egui::Separator::default());
                    }

                    ui_number_drag(ui, &mut rows, "Rows:");
                    ui_number_drag(ui, &mut cols, "Cols:");
                    ui_number_drag(ui, &mut x_val, "X Val:");
                });

            egui::Window::new("Running")
                .default_size([185.0, 1.0])
                .anchor(egui::Align2::LEFT_TOP, [0.0, settings_height])
                .show(egui_ctx, |ui| {
                    if DEBUG {
                        ui.label(format!("Width: {width}"));
                        ui.label(format!("Width Adj: {width_adj}"));
                        ui.label(format!("Height: {height}"));
                        ui.label(format!("Square Size: {square_size}"));

                        ui.label(format!("Window Size: {:?}", ui.available_size()));
                        ui.label(format!("Settings Height: {settings_height}"));
                        ui.add(egui::Separator::default());
                    }
                });
        });

        board.draw();

        egui_macroquad::draw();

        next_frame().await
    }
}
