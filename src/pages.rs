use egui_pages::{DisplayablePage, PageContainer, show_page};
use strum::{EnumIter, IntoEnumIterator as _};
use tracing::error;

pub use self::{
    about::UiAbout, egui_settings::UiEguiSettings, log_viewer::UiLogViewer, sample::UiSample,
};
use crate::{DataShared, Permission};
mod about;
mod egui_settings;
mod log_viewer;
mod sample;

mod private {
    #[derive(Default)]
    /// Used to make some trait methods private
    pub struct Token;
}

// TODO: Add an enum variant for pages you want to add. The compiler will guid
// you to where you need to update, just follow the pattern in those areas.

/// Records the types of possible pages
#[derive(Debug, serde::Serialize, serde::Deserialize, EnumIter)]
pub enum UiPage {
    Sample(UiSample),
    EguiSetting(UiEguiSettings),
    About(UiAbout),
    LogViewer(UiLogViewer),
}

impl egui_helpers::RemovableItem for UiPage {
    fn widget_text(&self) -> impl Into<egui::WidgetText> {
        self.title()
    }

    fn is_enabled(&self) -> bool {
        self.is_page_open()
    }

    fn set_enabled(&mut self, value: bool) {
        if value {
            self.open_page();
        } else {
            self.close_page();
        }
    }
}

macro_rules! do_on_ui_page {
    ($on:ident, $page:ident, $body:tt) => {
        match $on {
            UiPage::Sample($page) => $body,
            UiPage::EguiSetting($page) => $body,
            UiPage::About($page) => $body,
            UiPage::LogViewer($page) => $body,
        }
    };
}

impl PageContainer<DataShared, Permission, private::Token> for UiPage {
    #[tracing::instrument(ret)]
    fn new_page_with_unique_number<T: DisplayablePage<DataShared, Permission, private::Token>>(
        page_unique_number: usize,
    ) -> Self {
        for page in Self::iter() {
            if page.title_base() == T::title_base() {
                return match page {
                    Self::Sample(_) => {
                        Self::Sample(UiSample::new_page(page_unique_number).and_open_page())
                    }
                    Self::EguiSetting(_) => Self::EguiSetting(
                        UiEguiSettings::new_page(page_unique_number).and_open_page(),
                    ),
                    Self::About(_) => {
                        Self::About(UiAbout::new_page(page_unique_number).and_open_page())
                    }
                    Self::LogViewer(_) => {
                        Self::LogViewer(UiLogViewer::new_page(page_unique_number).and_open_page())
                    }
                };
            }
        }
        let msg = format!(
            "execution should never get here. All pages should be able to be found but {:?} not found",
            T::title_base()
        );
        error!("{msg}");
        unreachable!("{msg}");
    }

    fn display_page(&mut self, ui: &mut egui::Ui, data_shared: &mut DataShared) {
        do_on_ui_page!(self, page, { show_page(page, ui, data_shared) });
    }

    fn title_base(&self) -> &'static str {
        do_on_ui_page!(self, page, { page.title_base_from_instance() })
    }

    fn page_unique_number(&self) -> usize {
        do_on_ui_page!(self, page, { page.page_unique_number() })
    }

    fn is_page_open(&self) -> bool {
        do_on_ui_page!(self, page, { page.is_page_open() })
    }

    fn title(&self) -> String {
        do_on_ui_page!(self, page, { page.title() })
    }

    fn open_page(&mut self) {
        do_on_ui_page!(self, page, { page.open_page() });
    }

    fn close_page(&mut self) {
        do_on_ui_page!(self, page, { page.close_page() });
    }

    fn ui_menu_page_btn<T: DisplayablePage<DataShared, Permission, private::Token>>(
        ui: &mut egui::Ui,
        data_shared: &DataShared,
        active_pages: &mut Vec<Self>,
    ) {
        Self::internal_do_ui_menu_page_btn::<T>(ui, data_shared, active_pages);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_page_base_names_are_unique() {
        let mut set: HashSet<&str> = Default::default();
        for page in UiPage::iter() {
            let title_base = page.title_base();
            let is_unique = set.insert(title_base);
            assert!(
                is_unique,
                "Duplicate page title base name found: {title_base}"
            );
        }
    }
}
