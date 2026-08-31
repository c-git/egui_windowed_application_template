#[derive(Default, Debug)]
pub(crate) enum ModalKind {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub(crate) struct ModalInfo {
    pub(crate) text: String,
    pub(crate) kind: ModalKind,
    pub(crate) width: Option<f32>,
    pub(crate) show_copy_msg_button: bool,
}

impl ModalInfo {
    pub(crate) fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            kind: Default::default(),
            width: Default::default(),
            show_copy_msg_button: true,
        }
    }

    #[must_use]
    /// Convenience function to use builder pattern in argument place
    pub(crate) fn set_kind(self, kind: ModalKind) -> Self {
        Self { kind, ..self }
    }

    #[must_use]
    /// Convenience function to use builder pattern in argument place
    pub(crate) fn set_width(self, width: f32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    #[must_use]
    /// Convenience function to use builder pattern in argument place
    pub(crate) fn set_show_copy_msg_button(self, show_copy_msg_button: bool) -> Self {
        Self {
            show_copy_msg_button,
            ..self
        }
    }
}

impl ModalKind {
    pub(crate) fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Warning => "⚠",
            Self::Error => "❌",
        }
    }

    pub(crate) fn color(&self) -> egui::Color32 {
        match self {
            Self::Info => egui::Color32::LIGHT_BLUE,
            Self::Warning => egui::Color32::YELLOW,
            Self::Error => egui::Color32::RED,
        }
    }

    /// Returns `true` if the modal kind is [`Error`].
    ///
    /// [`Error`]: ModalKind::Error
    #[must_use]
    pub(crate) fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
