use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::action::Action;
use super::Entry;

#[derive(Serialize, Deserialize)]
pub struct Connection {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub action: Action,
    pub in_data_ids: Vec<u64>,
    pub out_data_ids: Vec<u64>,
    pub in_transform_ids: Vec<u64>,
    pub out_transform_ids: Vec<u64>
}

impl std::cmp::PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.action == other.action) &&
        (self.in_data_ids == other.in_data_ids) &&
        (self.out_data_ids == other.out_data_ids) &&
        (self.in_transform_ids == other.in_transform_ids) &&
        (self.out_transform_ids == other.out_transform_ids)
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Entry for Connection {
    fn get_id(&self) -> u64 {
        self.id
    }
}