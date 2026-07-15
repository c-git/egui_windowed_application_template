use crate::{DataShared, Permission, {{ app_struct_identifier }}, pages::private};
use egui::Ui;
use egui_pages::{DisplayablePage, displayable_page_common};
use tracing::info;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct UiEguiSettings {
    is_open: bool,
    page_unique_number: usize,
    #[serde(skip)]
    prev_ui_options: Option<egui::Options>,
}
impl UiEguiSettings {
    fn save_current_ui_options(&mut self, ui: &egui::Ui) {
        let current_ui_options = ui.options(|o| o.clone());
        self.prev_ui_options = Some(current_ui_options);
        let visuals = ui.global_style().visuals.clone();
        ui.data_mut(|w| w.insert_persisted(egui::Id::new({{ app_struct_identifier }}::VISUALS_KEY), visuals));
        info!("Saved UI Visuals");
    }
}

impl DisplayablePage<DataShared, Permission, private::Token> for UiEguiSettings {
    displayable_page_common!("UI Settings", &[], private::Token);

    fn show(&mut self, ui: &mut Ui, _data_shared: &mut DataShared) {
        let ctx = ui.ctx().clone();
        ctx.settings_ui(ui);
        match self.prev_ui_options.as_ref() {
            Some(prev) => {
                if ctx.options(|o| o != prev) {
                    self.save_current_ui_options(ui);
                }
            }
            None => self.save_current_ui_options(ui),
        }
    }
}
