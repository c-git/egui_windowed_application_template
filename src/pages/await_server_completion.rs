use std::ops::ControlFlow;

use egui_helpers::UiHelpers as _;
use reqwest_cross::{Awaiting, DataState, oneshot};

use crate::pages::error_helpers::ErrorStore as _;

/// Provides a way to track if a task is completed on the server by awaiting a
/// response. The caller is responsible for taking any post response actions
/// that are applicable such as reloading other dependent data and resetting the
/// instance of this struct so that it registers that no operations is still
/// ongoing. Not does not provide a way to access the data returned
#[derive(Debug)]
pub struct ServerOperationResponse<T = ()> {
    pub status: DataState<T>,
}

#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum OpOutcome {
    Completed,
    Ongoing,
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorState {
    IsReset,
    StillPresent,
}

#[must_use]
#[derive(Debug, PartialEq, Eq)]
pub enum OpResult {
    NoAction,
    ResetPage,
}

impl OpResult {
    /// Returns `true` if the op result is [`ResetPage`].
    ///
    /// [`ResetPage`]: OpResult::ResetPage
    #[must_use]
    pub fn is_reset_page(&self) -> bool {
        matches!(self, Self::ResetPage)
    }
}

impl OpOutcome {
    /// Returns `true` if the op outcome is [`Failed`].
    ///
    /// [`Failed`]: OpOutcome::Failed
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(..))
    }
}

impl<T> ServerOperationResponse<T> {
    /// Returns None if no operation is ongoing
    pub fn operation_outcome(&mut self) -> Option<OpOutcome> {
        match self.status.as_mut() {
            DataState::None => {
                // No action no save ongoing
                None
            }
            DataState::AwaitingResponse(rx) => {
                if let Some(new_state) = DataState::await_data(rx) {
                    self.status = new_state;
                }
                Some(OpOutcome::Ongoing)
            }
            DataState::Present(_data) => Some(OpOutcome::Completed),
            DataState::Failed(e) => {
                Some(OpOutcome::Failed(format!("Server Operation Failed: {e}")))
            }
        }
    }

    #[must_use]
    /// Shows a default UI for processing and returns the same value as
    /// [`Self::operation_outcome`] along with if the user reset the error
    /// message
    pub fn operation_outcome_with_ui(
        &mut self,
        ui: &mut egui::Ui,
    ) -> (Option<OpOutcome>, Option<ErrorState>) {
        let mut result = (self.operation_outcome(), None);
        if let Some(op_outcome) = result.0.as_ref() {
            // Save in progress
            match op_outcome {
                OpOutcome::Completed => {}
                OpOutcome::Ongoing => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Processing...");
                    });
                }
                OpOutcome::Failed(e) => {
                    ui.error_label(e);
                    if ui.button("Clear Error").clicked() {
                        *self = Self::default();
                        result.1 = Some(ErrorState::IsReset);
                    } else {
                        result.1 = Some(ErrorState::StillPresent);
                    }
                }
            }
        }
        result
    }

    /// Polls the outcome and indicates two things:
    /// - If pages that do not want to show the data during processing should
    ///   break
    /// - And if the page should be reloaded
    pub fn poll(&mut self, ui: &mut egui::Ui) -> ControlFlow<OpResult> {
        // TODO 4: Add automatic timeout
        let (outcome, error_state) = self.operation_outcome_with_ui(ui);
        debug_assert!(
            error_state.is_none() || error_state.is_some() == outcome.as_ref().is_some_and(|x|x.is_failed()),
            "if the error state is set that implies the current state was error
            (may no longer be if reset but that will not be reflected in the current outcome value)"
        );
        match outcome {
            Some(OpOutcome::Completed) => ControlFlow::Break(OpResult::ResetPage),
            Some(OpOutcome::Ongoing) => ControlFlow::Break(OpResult::NoAction),
            Some(OpOutcome::Failed(_)) => {
                match error_state.expect("this should always be some when in error state") {
                    ErrorState::IsReset => ControlFlow::Break(OpResult::ResetPage),
                    ErrorState::StillPresent => ControlFlow::Break(OpResult::NoAction),
                }
            }
            None => ControlFlow::Continue(()),
        }
    }

    // Disables the UI if there is an ongoing operation and returns if the UI should
    // be reset
    pub fn poll_with_disable(&mut self, ui: &mut egui::Ui) -> OpResult {
        match self.poll(ui) {
            ControlFlow::Continue(_) => OpResult::NoAction,
            ControlFlow::Break(op_result) => {
                // Operation in Progress, disable any UI following while operation is ongoing
                ui.disable();
                op_result
            }
        }
    }

    /// Providers a receiver to be monitored
    pub fn monitor_receiver(&mut self, rx: oneshot::Receiver<Result<T, anyhow::Error>>) {
        self.status = DataState::AwaitingResponse(Awaiting(rx));
    }

    /// Allows reusing this struct to store errors
    pub fn set_error_state_from_anyhow<E: Into<anyhow::Error>>(&mut self, err: E) {
        self.status.set_error_state_from_anyhow(err);
    }

    /// Allows reusing this struct to store errors
    pub fn set_error_state_from_str<S: AsRef<str>>(&mut self, s: S) {
        self.status.set_error_state_from_str(s);
    }
}

impl<T> Default for ServerOperationResponse<T> {
    fn default() -> Self {
        Self {
            status: Default::default(),
        }
    }
}
