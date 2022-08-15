use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::directory::Directory;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub data_dir: Directory
}
