use crate::{
    DataShared, UiPage,
    pages::{self, UiAbout, UiSample},
    shortcuts::Shortcuts,
};
use egui_pages::PageContainer as _;
use std::hash::{Hash as _, Hasher as _};
use tracing::{debug, error, info};
use wykies_time::Timestamp;

const VERSION_STR: &str = concat!("ver: ", env!("CARGO_PKG_VERSION"));

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    data_shared: DataShared,
    active_pages: Vec<UiPage>,
    shortcuts: Shortcuts,
    #[serde(skip)]
    last_save_hash: Option<u64>,
}

impl TemplateApp {
    pub const VISUALS_KEY: &str = "visuals";

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown and periodically
    /// as on web there is no shutdown notification
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.is_changed_since_last_save() {
            info!("Saving with key: {}", eframe::APP_KEY);
            eframe::set_value(storage, eframe::APP_KEY, self);
        } else {
            debug!("Not saving as no change has been detected");
        }
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // For inspiration and more examples, go to https://emilk.github.io/egui
        // For a simpler example see https://github.com/emilk/eframe_template which this template expands on
        // Create pages to add your widgets to. See the TODO comments across the code

        self.data_shared.screen_lock_info.tick();
        self.top_panel(ui);
        self.bottom_panel(ui);
        self.show_pages(ui);

        // Request repaint after 1 second
        ui.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

impl TemplateApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                self.ui_menu_file(ui);
                self.ui_menu_pages(ui);

                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);

                ui.add_space(16.0);
                ui.label(VERSION_STR);
            });
        });
    }

    fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                ui.label(Self::current_time());
                self.ui_lock_info(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn show_pages(&mut self, ui: &mut egui::Ui) {
        UiPage::ui_active_pages_panel(ui, &mut self.active_pages, &self.shortcuts.organize_pages);
        UiPage::ui_display_pages(ui, &mut self.active_pages, &mut self.data_shared);
    }

    fn current_time() -> String {
        Timestamp::now().display_as_utc_datetime_long()
    }

    fn ui_menu_file(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            UiPage::ui_menu_page_btn::<pages::UiEguiSettings>(
                ui,
                &self.data_shared,
                &mut self.active_pages,
            );

            // On the web the browser controls the zoom
            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.separator();
                egui::gui_zoom::zoom_menu_buttons(ui);
                ui.weak(format!("Current zoom: {:.0}%", 100.0 * ui.zoom_factor()))
                    .on_hover_text(
                        "The UI zoom level, on top of the operating system's default value",
                    );
                ui.separator();
            }
            UiPage::ui_menu_page_btn::<UiAbout>(ui, &self.data_shared, &mut self.active_pages);

            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            if ui.button("Quit").clicked() {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn is_changed_since_last_save(&mut self) -> bool {
        let as_ron = match ron::to_string(&self) {
            Ok(s) => s,
            Err(err_msg) => {
                error!("{err_msg:?}");
                return true;
            }
        };
        let mut hasher = std::hash::DefaultHasher::new();
        as_ron.hash(&mut hasher);
        let new_hash = hasher.finish();
        if let Some(&old_hash) = self.last_save_hash.as_ref()
            && old_hash == new_hash
        {
            return false;
        }
        self.last_save_hash = Some(new_hash);
        true
    }

    fn ui_menu_pages(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Pages", |ui| {
            UiPage::ui_menu_page_btn::<pages::UiSample>(
                ui,
                &self.data_shared,
                &mut self.active_pages,
            );
        });
    }

    fn ui_lock_info(&mut self, ui: &mut egui::Ui) {
        let is_locked = self.data_shared.screen_lock_info.is_locked();
        let locked = if is_locked { "LOCKED" } else { "UNLOCKED" };
        let lock_at = self.data_shared.screen_lock_info.client_idle_timeout();
        let idle_time = self
            .data_shared
            .screen_lock_info
            .elapsed_time_since_user_activity();

        ui.separator();
        ui.label(format!("Idle time: {idle_time:}, Set to pretend lock at {lock_at} Seconds, Hypothetical lock status is: {locked}"))
        .on_hover_text(
            "Locking not implemented but just showing here for demonstration purposes",
        );
        if ui
            .add_enabled(is_locked, egui::Button::new("Simulate Unlock"))
            .clicked()
        {
            self.data_shared.screen_lock_info.unlock();
        }
        ui.separator();
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        // Preload `active_pages` with a few pages for first open
        Self {
            data_shared: Default::default(),
            active_pages: vec![
                UiPage::new_page_with_unique_number::<UiSample>(0),
                UiPage::new_page_with_unique_number::<UiAbout>(0),
            ],
            shortcuts: Default::default(),
            last_save_hash: Default::default(),
        }
    }
}
