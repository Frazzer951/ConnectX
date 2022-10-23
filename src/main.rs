use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Connect X".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_primitives() {
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
}

#[macroquad::main(window_conf)]
async fn main() {
    let num_rows = &mut 1;

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                ui.add(egui::Slider::new(num_rows, 1..=10).text("Number of Rows"));

                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        *num_rows -= 1;
                    }
                    ui.label(num_rows.to_string());
                    if ui.button("+").clicked() {
                        *num_rows += 1;
                    }
                });
            });
        });

        draw_primitives();
        draw_text(
            format!("Num Rows: {}", num_rows).as_str(),
            500.0,
            500.0,
            30.0,
            DARKGRAY,
        );

        egui_macroquad::draw();

        next_frame().await
    }
}
