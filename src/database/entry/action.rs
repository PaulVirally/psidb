use serde::{Serialize, Deserialize};

pub enum Action {
    Apply, // Applies a transformation to a data entry
    Chain, // Chains multiple transformations together
    Link // Links multiple data entries together
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        match self {
            Action::Apply => serializer.serialize_str("apply"),
            Action::Chain => serializer.serialize_str("chain"),
            Action::Link => serializer.serialize_str("link")
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Action, D::Error>
        where D: serde::Deserializer<'de>
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match &*s {
            "apply" => Ok(Action::Apply),
            "chain" => Ok(Action::Chain),
            "link" => Ok(Action::Link),
            _ => Err(serde::de::Error::custom(format!("Unknown action: {}", s)))
        }
    }
}