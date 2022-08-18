use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::Entry;

#[derive(Serialize, Deserialize)]
pub struct Transform {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub script_paths: Vec<String>,
    pub script_args: Vec<Option<String>>,
    pub script_git_hashes: Vec<Option<String>>
}

impl std::cmp::PartialEq for Transform {
    fn eq(&self, other: &Transform) -> bool {
        (self.script_paths == other.script_paths) &&
        (self.script_args == other.script_args) &&
        (self.script_git_hashes == other.script_git_hashes)
    }

    fn ne(&self, other: &Transform) -> bool {
        !self.eq(other)
    }
}

impl Entry for Transform {
    fn get_id(&self) -> u64 {
        self.id
    }
}