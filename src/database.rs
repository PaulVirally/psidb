pub mod entry;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use entry::{data::Data, transformation::Transformation, connection::Connection, directory::Directory};
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

    pub fn add_data(&mut self, data_dir: Directory, meta_data: Option<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = meta_data.unwrap_or(HashMap::new());
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let data = Data {
            id: self.curr_id,
            md: md,
            data_dir: data_dir
        };
        self.data_vec.push(data);
        self.curr_id += 1;

        Ok(())
    }

    pub fn add_transformation(&mut self, script_paths: Vec<String>, script_args: Vec<Option<Vec<String>>>, script_git_hashes: Vec<Option<String>>, meta_data: Option<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
        // Make sure that the script_paths, script_args, and script_git_hashes are the same length
        if script_paths.len() != script_args.len() || script_paths.len() != script_git_hashes.len() {
            return Err("script_paths, script_args, and script_git_hashes must be the same length".into());
        }

        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = meta_data.unwrap_or(HashMap::new());
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        // Get the hashes of the scripts if they are not provided (or None if they are not tracked by git)
        let mut used_hashes: Vec<Option<String>> = vec![None; script_paths.len()];
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
                used_hashes[i] = {
                    if commit.is_err() {
                        println!("Warning: Ignoring hash {} for {} because it is not a valid commit", path_str, hash);
                        None
                    }
                    else {
                        Some(hash.to_owned())
                    }
                };
                continue;
            }

            // Get the hash of the latest commit
            used_hashes[i] = Some(String::from_utf8(repo.head()?.target().unwrap().as_bytes().to_vec())?);
        }

        let transformation = Transformation {
            id: self.curr_id,
            md: md,
            script_paths: script_paths,
            script_args: script_args,
            script_git_hashes: used_hashes
        };
        self.transformation_vec.push(transformation);
        self.curr_id += 1;
        Ok(())
    }
}