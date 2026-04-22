use super::DisplayablePage;
use crate::{DataShared, displayable_page_common};
use egui::Ui;

const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct UiAbout {
    is_open: bool,
    page_unique_number: usize,
}

impl DisplayablePage for UiAbout {
    displayable_page_common!("About");

    fn show(&mut self, ui: &mut Ui, _data_shared: &mut DataShared) {
        egui::Grid::new(self.unique_prefix_for_id(&self.unique_prefix_for_id("grid")))
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // Shows a few of the environment variables set by cargo during compilation. For
                // the full list of variables cargo sets see
                // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

                ui.label("Authors");
                ui.label(AUTHORS);
                ui.end_row();
                ui.label("Name");
                ui.label(NAME);
                ui.end_row();
                ui.label("Description");
                ui.label(DESCRIPTION);
                ui.end_row();
                ui.label("Version");
                ui.label(VERSION);
                ui.end_row();
            });
        ui.add(egui::github_link_file!(
            "https://github.com/c-git/egui_windowed_application_template/blob/main/",
            "Source code."
        ));
    }
}
