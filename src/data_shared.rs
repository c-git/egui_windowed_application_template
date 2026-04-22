use crate::{
    Permission,
    consts::{CLIENT_IDLE_TIMEOUT, CLIENT_TICKS_PER_SECOND_FOR_ACTIVE},
};
use egui_helpers::ScreenLockInfo;
use egui_pages::PermissionValidator;

/// Passed to all pages, intended to store info that all would need access to
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataShared {
    /// For the sake of simplicity I've not wrapped the API of this field but
    /// you can easily put wrappers around it and not need to make it pub.
    /// However, since it's only here for demonstration purposes I've made it as
    /// easy as possible to remove.
    #[serde(skip)]
    pub screen_lock_info: ScreenLockInfo,
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
        }
    }
}
