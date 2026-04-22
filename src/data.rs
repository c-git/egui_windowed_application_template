use egui_pages::PermissionValidator;

use crate::Permission;

/// Passed to all pages, intended to store info that all would need access to
#[derive(Debug, Default)]
pub struct DataShared;

impl PermissionValidator<Permission> for DataShared {
    fn has_permissions(&self, _required_permissions: &[Permission]) -> bool {
        // For an example of an actual use of this function see
        // https://github.com/wykies/crates/blob/eb6bd6030990ee1bc95059886e1c79d86fecdfc2/crates/chat-app-client/src/app.rs#L78
        true
    }
}
