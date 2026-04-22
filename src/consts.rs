//! Provides an easy place to place constant (setting that we want out of the
//! code but not worth putting in a config)

use wykies_time::Seconds;

pub const CLIENT_IDLE_TIMEOUT: Seconds = Seconds::new(30);
pub const CLIENT_TICKS_PER_SECOND_FOR_ACTIVE: usize = 5;
