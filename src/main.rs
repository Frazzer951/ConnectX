use macroquad::prelude::*;

const DEBUG: bool = true;

fn window_conf() -> Conf {
    Conf {
        window_title: "Connect X".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn ui_number_drag(ui: &mut egui::Ui, val: &mut i32, text: &str) {
    ui.columns(2, |columns| {
        columns[0].label(text);
        columns[1].add(egui::DragValue::new(val));
    });
}

fn draw_squares(size: f32, rows: i32, cols: i32) {
    for i in 0..rows {
        for j in 0..cols {
            draw_rectangle(
                j as f32 * size,
                i as f32 * size,
                size - 1.0,
                size - 1.0,
                GRAY,
            );
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rows: i32 = 6;
    let mut cols: i32 = 7;
    let mut x_val: i32 = 4;

    loop {
        let width: f32 = screen_width();
        let height: f32 = screen_height();
        let square_size = (width / cols as f32).min(height / rows as f32);

        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .default_size([1.0, 1.0])
                .show(egui_ctx, |ui| {
                    if DEBUG {
                        ui.label(format!("Width: {}", width));
                        ui.label(format!("Height: {}", height));
                        ui.label(format!("Square Size: {}", square_size));
                        ui.add(egui::Separator::default());
                    }

                    ui_number_drag(ui, &mut rows, "Rows:");
                    ui_number_drag(ui, &mut cols, "Cols:");
                    ui_number_drag(ui, &mut x_val, "X Val:");
                });
        });

        draw_squares(square_size, rows, cols);

        egui_macroquad::draw();

        next_frame().await
    }
}
