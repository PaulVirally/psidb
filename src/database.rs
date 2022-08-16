pub mod entry;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use entry::{data::Data, transformation::Transformation, connection::Connection, action::Action};
use chrono::{Utc, SecondsFormat};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub data_vec: Vec<Data>,
    pub transformation_vec: Vec<Transformation>,
    pub connection_vec: Vec<Connection>,
    curr_id: u64
}

impl Database {
    pub fn new() -> Database {
        Database {
            data_vec: Vec::new(),
            transformation_vec: Vec::new(),
            connection_vec: Vec::new(),
            curr_id: 0
        }
    }

    pub fn add_data(&mut self, data_paths: Vec<String>, meta_data: Option<HashMap<String, String>>) -> Result<u64, Box<dyn Error>> {
        // Check if the paths are valid and make the paths aboslute paths
        let mut used_paths: Vec<String> = vec!["".to_owned(); data_paths.len()];
        for (i, path) in data_paths.iter().enumerate() {
            if Path::new(path).exists() {
                used_paths[i] = Path::new(path).canonicalize()?.to_str().unwrap().to_owned();
            }
            else {
                return Err(format!("Path {} does not exist", path).into());
            }
        }

        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = meta_data.unwrap_or(HashMap::new());
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let data = Data {
            id: self.curr_id,
            md: md,
            data_paths: used_paths
        };
        self.data_vec.push(data);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn add_transformation(&mut self, script_paths: Vec<String>, script_args: Vec<Option<Vec<String>>>, script_git_hashes: Vec<Option<String>>, meta_data: Option<HashMap<String, String>>) -> Result<u64, Box<dyn Error>> {
        // Make sure that the script_paths, script_args, and script_git_hashes are the same length
        if script_paths.len() != script_args.len() || script_paths.len() != script_git_hashes.len() {
            return Err("script_paths, script_args, and script_git_hashes must be the same length".into());
        }

        // Get the hashes of the scripts if they are not provided (or None if they are not tracked by git), and get the absolute paths of the scripts
        let mut used_hashes: Vec<Option<String>> = vec![None; script_paths.len()];
        let mut used_paths: Vec<String> = vec!["".to_owned(); script_paths.len()];
        for (i, (path_str, hash)) in script_paths.iter().zip(script_git_hashes).enumerate() {
            let path = Path::new(path_str);

            // Make sure the script exists and is a file
            if !path.exists() {
                return Err(format!("Script path {} does not exist", path_str).into());
            }

            // Make sure the script is a file
            if !path.is_file() {
                return Err(format!("Script path {} is not a file", path_str).into());
            }

            // Get the absolute path of the script
            used_paths[i] = path.canonicalize()?.to_str().unwrap().to_owned();

            // Check if the file is in a git repository
            let parent_dir = path.parent().unwrap_or(Path::new("/"));
            let repo = git2::Repository::discover(parent_dir);
            if !repo.is_ok() {
                used_hashes[i] = None;
                continue;
            }
            let repo = repo?;

            if hash.is_some() {
                // Check if the hash is a valid commit
                let hash = hash.unwrap();
                let oid = git2::Oid::from_str(hash.as_str())?;
                let commit = repo.find_commit(oid);
                if commit.is_err() {
                    println!("Warning: Ignoring hash {} for {} because it is not a valid commit", hash, path_str);
                }
                else {
                    used_hashes[i] = Some(hash);
                    continue;
                }
            }

            // Get the hash of the latest commit
            let target = repo.head()?.target();
            if target.is_some() {
                // If the repository has a head, get the hash of the latest commit
                used_hashes[i] = Some(target.unwrap().to_string());
                continue;
            }
            used_hashes[i] = None;
        }

        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = meta_data.unwrap_or(HashMap::new());
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let transformation = Transformation {
            id: self.curr_id,
            md: md,
            script_paths: used_paths,
            script_args: script_args,
            script_git_hashes: used_hashes
        };
        self.transformation_vec.push(transformation);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn add_connection(&mut self, action: Action, data_ids: Vec<u64>, transformation_ids: Vec<u64>, meta_data: Option<HashMap<String, String>>) -> Result<u64, Box<dyn Error>> {
        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = meta_data.unwrap_or(HashMap::new());
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let connection = Connection {
            id: self.curr_id,
            md: md,
            action: action,
            data_ids: data_ids,
            transformation_ids: transformation_ids
        };
        self.connection_vec.push(connection);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }
}