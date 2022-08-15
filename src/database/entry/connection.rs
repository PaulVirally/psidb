use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::action::Action;

#[derive(Serialize, Deserialize)]
pub struct Connection {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub action: Action,
    pub data_ids: Vec<u64>,
    pub transformation_ids: Vec<u64>,
}