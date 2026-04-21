/// TODO: Review this page and use as an example
use super::DisplayablePage;
use crate::{DataShared, displayable_page_common};
use egui::Ui;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UiSample {
    is_open: bool,
    page_unique_number: usize,

    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for UiSample {
    fn default() -> Self {
        Self {
            is_open: Default::default(),
            page_unique_number: Default::default(),

            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl DisplayablePage for UiSample {
    displayable_page_common!("UI Sample");

    fn show(&mut self, ui: &mut Ui, _data_shared: &mut DataShared) {
        // You are fine to use panels in here if you want but as the ui is already in
        // the context of a window it isn't always needed. The panel has only been left
        // here as an example.
        //
        // If you use a central panel it takes up the region left after adding
        // TopPanel's and SidePanel's. Which also means that it needs to be created
        // after them in the code.
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
