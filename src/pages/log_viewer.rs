/// TODO: Review this page and use as an example.
/// It recreates the starting page in the original template
use crate::{DataShared, Permission, pages::private};
use egui::{Panel, Ui};
use egui_pages::{DisplayablePage, displayable_page_common};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UiLogViewer {
    is_open: bool,
    page_unique_number: usize,
    msg: String,
}

impl DisplayablePage<DataShared, Permission, private::Token> for UiLogViewer {
    displayable_page_common!("Log Viewer", &[], private::Token);

    fn show(&mut self, ui: &mut Ui, data_shared: &mut DataShared) {
        // TODO: Remove sample top panel for testing tracing
        Panel::top(self.unique_prefix_for_id("top")).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Message");
                ui.text_edit_singleline(&mut self.msg);
            });
            ui.horizontal(|ui| {
                ui.label("Add as");
                if ui.button("Trace").clicked() {
                    tracing::trace!("{}", self.msg);
                }
                if ui.button("Debug").clicked() {
                    tracing::debug!("{}", self.msg);
                }
                if ui.button("Info").clicked() {
                    tracing::info!("{}", self.msg);
                }
                if ui.button("Warn").clicked() {
                    tracing::warn!("{}", self.msg);
                }
                if ui.button("Error").clicked() {
                    tracing::error!("{}", self.msg);
                }
            });
        });

        ui.add(egui_tracing::Logs::new(
            data_shared.egui_tracing_collector.clone(),
        ));
    }
}

impl Default for UiLogViewer {
    fn default() -> Self {
        Self {
            is_open: Default::default(),
            page_unique_number: Default::default(),
            msg: "Sample Log Message".to_owned(),
        }
    }
}
