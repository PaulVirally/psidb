use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::utils;
use super::Entry;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub paths: Vec<String>
}

impl std::cmp::PartialEq for Data {
    fn eq(&self, rhs: &Data) -> bool {
        utils::is_permutation_small(&self.paths, &rhs.paths)
    }

    fn ne(&self, rhs: &Data) -> bool {
        !self.eq(rhs)
    }
}

impl Entry for Data {
    fn get_id(&self) -> u64 {
        self.id
    }
}