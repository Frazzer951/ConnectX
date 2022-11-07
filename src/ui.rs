use std::ops::RangeInclusive;

use crate::agent::Agent;

pub fn number_drag(ui: &mut egui::Ui, val: &mut usize, text: &str, range: RangeInclusive<usize>) {
    ui.columns(2, |columns| {
        columns[0].label(text);
        columns[1].add(egui::DragValue::new(val).clamp_range(range));
    });
}

pub fn agent_selector(ui: &mut egui::Ui, text: &str, agent: &mut Agent) {
    ui.columns(2, |columns| {
        columns[0].label(text);

        egui::ComboBox::from_id_source(text)
            .selected_text(format!("{:?}", agent))
            .show_ui(&mut columns[1], |ui| {
                ui.selectable_value(agent, Agent::Player, "Player");
                ui.selectable_value(agent, Agent::Random, "Random");
            });
    });
}
