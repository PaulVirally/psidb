use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Directory {
    pub fpaths: Vec<String>,
    pub sub_dirs: Vec<Directory>
}