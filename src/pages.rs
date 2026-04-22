use strum::{EnumIter, IntoEnumIterator as _};
use tracing::{error, info};

pub use self::{about::UiAbout, egui_settings::UiEguiSettings, sample::UiSample};
use crate::DataShared;
mod about;
mod egui_settings;
mod macros;
pub mod sample;

mod private {
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

/// Trait for types that can be treated as pages to display
///
/// It has Default and serde Traits as super traits to ensure all these types
/// implement these traits
pub trait DisplayablePage: Default + serde::Serialize + serde::de::DeserializeOwned {
    /// Reset the state of the screen
    fn reset_to_default(&mut self, _: private::Token) {
        // By default serialize and deserialize to reset
        let data = ron::to_string(self).expect("failed serialize to ron for reset");
        *self = ron::from_str(&data).expect("failed deserialize ron during reset");
    }

    /// Displays the page
    fn show(&mut self, ui: &mut eframe::egui::Ui, data_shared: &mut DataShared);

    /// Base of the page's title (numbers get appended to duplicates)
    ///
    /// ASSUMPTION: THIS IS UNIQUE PER TYPE
    fn title_base() -> &'static str;

    /// Convenance function for working with instances inside of the enum
    fn title_base_from_instance(&self) -> &'static str {
        Self::title_base()
    }

    /// Page number to make title unique
    ///
    /// Assumed that the caller will ensure this number is unique across pages
    /// with the same base title
    fn page_unique_number(&self) -> usize;

    /// Creates a page with the unique number passed
    fn new_page(page_unique_number: usize) -> Self;

    /// Pages display title (includes page number if not first)
    fn title(&self) -> String {
        if self.page_unique_number() == 0 {
            Self::title_base().to_owned()
        } else {
            format!("{} ({})", Self::title_base(), self.page_unique_number())
        }
    }

    /// Provides a consistent way to generate IDs that are unique throughout the
    /// application
    ///
    /// Needed to prevent duplicate ID if multiple of the same window are used
    /// and not need to be aware of the global namespace for panels or other
    /// controls that can have conflict. Provides a prefix as it my be used by
    /// called functions and not have direct access to this method.
    ///
    /// # Precondition
    ///
    /// `id_name` is unique for the page on which it is provided or will be
    /// joined with something that is unique on a subpage
    ///
    /// # Assumptions
    ///
    /// - `Self::title` is unique throughout the application
    fn unique_prefix_for_id(&self, id_name: &str) -> String {
        format!("{}{id_name}", self.title())
    }

    fn is_page_open(&self) -> bool;

    fn open_page(&mut self) {
        info!("Open Page {}", self.title());
        self.internal_do_open_page(private::Token {});
    }

    fn close_page(&mut self) {
        info!("Close Page {}", self.title());
        self.internal_do_close_page(private::Token {});
    }

    fn internal_do_open_page(&mut self, _: private::Token);

    /// This usually clears any state loaded from the database
    fn internal_do_close_page(&mut self, _: private::Token);

    /// Convenance method for chaining
    #[must_use]
    fn and_open_page(mut self) -> Self {
        self.open_page();
        self
    }

    /// Provides an opportunity for the page to change settings on the window
    /// before display
    fn adjust_window_settings<'open>(&self, window: egui::Window<'open>) -> egui::Window<'open> {
        // Provide identity default impl
        window
    }
}

macro_rules! do_on_ui_page {
    ($on:ident, $page:ident, $body:tt) => {
        match $on {
            UiPage::Sample($page) => $body,
            UiPage::EguiSetting($page) => $body,
            UiPage::About($page) => $body,
        }
    };
}

impl UiPage {
    #[tracing::instrument(ret)]
    pub fn new_page_with_unique_number<T: DisplayablePage>(page_unique_number: usize) -> Self {
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

    pub fn display_page(&mut self, ctx: &egui::Context, data_shared: &mut DataShared) {
        do_on_ui_page!(self, page, { show_page(page, ctx, data_shared) });
    }

    pub fn title_base(&self) -> &'static str {
        do_on_ui_page!(self, page, { page.title_base_from_instance() })
    }

    pub fn page_unique_number(&self) -> usize {
        do_on_ui_page!(self, page, { page.page_unique_number() })
    }

    pub fn is_page_open(&self) -> bool {
        do_on_ui_page!(self, page, { page.is_page_open() })
    }

    pub fn title(&self) -> String {
        do_on_ui_page!(self, page, { page.title() })
    }

    pub fn open_page(&mut self) {
        do_on_ui_page!(self, page, { page.open_page() });
    }

    pub fn close_page(&mut self) {
        do_on_ui_page!(self, page, { page.close_page() });
    }
}

fn show_page<P: DisplayablePage>(page: &mut P, ctx: &egui::Context, data_shared: &mut DataShared) {
    let mut is_open = page.is_page_open();
    if !is_open {
        return;
    }
    let mut window = egui::Window::new(page.title()).vscroll(true).hscroll(true);
    window = page.adjust_window_settings(window);
    window
        .open(&mut is_open)
        .show(ctx, |ui| page.show(ui, data_shared));
    if !is_open {
        page.close_page();
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
