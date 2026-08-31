use crate::{
    Permission,
    consts::{CLIENT_IDLE_TIMEOUT, CLIENT_TICKS_PER_SECOND_FOR_ACTIVE},
    data_shared::modal::ModalInfo,
};
use egui_helpers::ScreenLockInfo;
use egui_pages::PermissionValidator;

pub(crate) mod modal;

/// Passed to all pages, intended to store info that all would need access to
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataShared {
    /// For the sake of simplicity I've not wrapped the API of this field but
    /// you can easily put wrappers around it and not need to make it pub.
    /// However, since it's only here for demonstration purposes I've made it as
    /// easy as possible to remove.
    #[serde(skip)]
    pub screen_lock_info: ScreenLockInfo, /* TODO: Remove this field if you do not want locking
                                           * and just follow the compiler errors */
    #[serde(skip)]
    pub egui_tracing_collector: egui_tracing::EventCollector,

    #[serde(skip)]
    modal_msg: Option<ModalInfo>,
}

impl PermissionValidator<Permission> for DataShared {
    fn has_permissions(&self, _required_permissions: &[Permission]) -> bool {
        // For an example of an actual use of this function see
        // https://github.com/wykies/crates/blob/eb6bd6030990ee1bc95059886e1c79d86fecdfc2/crates/chat-app-client/src/app.rs#L78
        true
    }
}

impl Default for DataShared {
    fn default() -> Self {
        Self {
            screen_lock_info: ScreenLockInfo::new(
                CLIENT_IDLE_TIMEOUT,
                CLIENT_TICKS_PER_SECOND_FOR_ACTIVE,
            ),
            egui_tracing_collector: Default::default(),
            modal_msg: Default::default(),
        }
    }
}

impl DataShared {
    pub(crate) fn check_modal(&mut self, ui: &egui::Ui) {
        let Some(msg) = self.modal_msg.as_ref() else {
            return;
        };
        let modal = egui::Modal::new(egui::Id::new("global msg modal")).show(ui.ctx(), |ui| {
            let (mut width, max_height) = ui.ctx().input(|i| {
                let content_rect = i.content_rect();
                (content_rect.width() * 0.8, content_rect.height() * 0.8)
            });
            width = width.min(msg.width.unwrap_or(f32::INFINITY));
            ui.set_width(width);

            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(msg.kind.icon())
                        .size(24.0)
                        .color(msg.kind.color()),
                );
            });
            egui::ScrollArea::vertical()
                .max_height(max_height)
                .show(ui, |ui| {
                    if !msg.kind.is_error() {
                        ui.vertical_centered(|ui| {
                            ui.label(msg.text.as_str());
                        });
                    } else {
                        ui.label(msg.text.as_str());
                    }
                });

            ui.add_space(32.0);

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Close").clicked() {
                        ui.close();
                    }

                    if msg.show_copy_msg_button && ui.button("Copy message to clipboard").clicked()
                    {
                        ui.copy_text(msg.text.clone());
                        // TODO 1: Enable after we add toast support
                        // self.toast_add_queue.confirm_copy("Message");
                        ui.close();
                    }
                },
            );
        });

        if modal.should_close() {
            self.modal_msg = None;
        }
    }

    pub(crate) fn set_modal(&mut self, msg: ModalInfo) {
        self.modal_msg = Some(msg);
    }
}
