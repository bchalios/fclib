//! CpuTemplate : The CPU Template defines a set of flags to be disabled from the microvm so
//! that the features exposed to the guest are the same as in the selected instance type. Works
//! only on Intel.
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::ValueEnum;

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum CpuTemplate {
    C3,
    T2,
    T2S,
    T2CL,
    T2A,
    #[default]
    None,
}
