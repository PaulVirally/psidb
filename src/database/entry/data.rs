use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub data_paths: Vec<String>
}
