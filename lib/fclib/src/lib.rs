pub mod client;
pub mod vmm;

use semver::Version;

static FC_VERSION: &str = "1.4.0";

pub fn supported_fc_version() -> Version {
    Version::parse(FC_VERSION).unwrap()
}
