use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use egui::WidgetText;
use egui_toast::{Toast, Toasts};
use tracing::error;

/// Exists to make adding toasts easy to avoid any lifetime issues or other
/// current immutable borrows.
///
/// It is cheap to close as add it's data is wrapped in a Arc
#[derive(Default, Clone)]
pub(crate) struct ToastAddQueue {
    queue: Arc<Mutex<Vec<Toast>>>,
}

impl ToastAddQueue {
    /// Adds a new toast to the queue to be added to the `Toasts`
    fn do_add(&self, toast: Toast) {
        self.queue.lock().expect("lock poisoned").push(toast);
    }

    pub(crate) fn error(&self, err_msg: &anyhow::Error) {
        error!(?err_msg, "{err_msg:?}");
        let toast = egui_toast::Toast {
            text: err_msg.to_string().into(),
            kind: egui_toast::ToastKind::Error,
            options: egui_toast::ToastOptions::default()
                .duration_in_seconds(60.)
                .show_progress(true),
            ..Default::default()
        };
        self.do_add(toast);
    }

    pub(crate) fn confirm_copy<S: Display>(&self, title: S) {
        let toast = egui_toast::Toast {
            text: format!("{title} copied to clipboard").into(),
            kind: egui_toast::ToastKind::Success,
            options: egui_toast::ToastOptions::default()
                .duration_in_seconds(15.)
                .show_progress(true),
            ..Default::default()
        };
        self.do_add(toast);
    }

    pub(crate) fn confirm<S: Into<WidgetText>>(&self, text: S) {
        let toast = egui_toast::Toast {
            text: text.into(),
            kind: egui_toast::ToastKind::Success,
            options: egui_toast::ToastOptions::default()
                .duration_in_seconds(15.)
                .show_progress(true),
            ..Default::default()
        };
        self.do_add(toast);
    }

    pub(crate) fn info<S: Into<WidgetText>>(&self, text: S) {
        let toast = egui_toast::Toast {
            text: text.into(),
            kind: egui_toast::ToastKind::Info,
            options: egui_toast::ToastOptions::default()
                .duration_in_seconds(30.)
                .show_progress(true),
            ..Default::default()
        };
        self.do_add(toast);
    }

    /// Adds the toasts stored to `Toasts` in the order they were added
    pub(crate) fn dequeue_toasts(&self, toasts: &mut Toasts) {
        for toast in self.queue.lock().expect("lock poisoned").drain(..) {
            toasts.add(toast);
        }
    }
}
