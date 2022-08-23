use clap::ValueEnum;
use strum_macros::EnumString;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, ValueEnum)]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Apply, // Applies a transform to a data entry
    Chain, // Chains multiple transforms together
    Link // Links multiple data entries together
}