use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transformation {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub script_paths: Vec<String>,
    pub script_args: Vec<Option<Vec<String>>>,
    pub script_git_hashes: Vec<Option<String>> // Use git2::Repository::find_blob(Oid::from_str(&hash).unwrap()).unwrap().as_object() to get the actual git2::Commit or something
}