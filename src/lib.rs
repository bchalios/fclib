pub mod api;
pub mod instance;

pub use instance::{FcVmm, FcVmmBuilder, LogLevel};

use semver::Version;

static FC_VERSION: &str = "1.3.0";

pub fn supported_fc_version() -> Version {
    Version::parse(FC_VERSION).unwrap()
}
