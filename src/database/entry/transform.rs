use std::process::Command;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use itertools::izip;
use regex;
use super::Entry;
use super::data::Data;
use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct Transform {
    pub id: u64,
    pub md: HashMap<String, String>,
    pub script_paths: Vec<String>,
    pub script_args: Vec<Option<String>>,
    pub script_git_hashes: Vec<Option<String>>
}

impl Transform {
    pub fn apply(&self, data: &[&Data], id: u64) -> Result<Data, Box<dyn std::error::Error>> {
        // Parser for the script otuput
        let re = regex::bytes::Regex::new(r"psidb::out_path (.*)")?;

        // The paths for the first script
        let mut data_paths = if data.len() > 0 {
            data.first().unwrap().paths.clone()
        } else {
            vec![]
        };

        // Apply each script sequentially
        for (path, args, hash_str) in izip!(&self.script_paths, &self.script_args, &self.script_git_hashes) {
            // Find the repository of the script if it exists
            let script_dir = std::path::Path::new(&path).parent().unwrap_or_else(|| std::path::Path::new("/"));
            let mut repo = git2::Repository::discover(script_dir);
            let using_git = repo.is_ok() && hash_str.is_some();

            // Checkout the hash if we have one
            let checkout_res = if using_git {
                let hash_oid = git2::Oid::from_str(hash_str.as_ref().unwrap())?;
                Some(utils::safe_git_checkout_commit(repo.as_mut().unwrap(), hash_oid)?)
            } else {
                None
            };

            // Make sure the script exists and is a file
            utils::verify_file_path(&path)?;

            // Construct the args to pass to the script
            let mut passed_args: Vec<&str> = data_paths.iter().map(AsRef::as_ref).collect();
            let derefed_args = args.as_ref().map(AsRef::as_ref).unwrap_or_default();
            let parsed_args = utils::parse_args(derefed_args, id);
            passed_args.push(&parsed_args);
            println!("Parsed args: {:?}", parsed_args);
            println!("Passing args: {:?}", passed_args);

            // Run the script with the args provided
            let output = Command::new(path).args(passed_args).output()?;
            println!("Command: {:?}", path);
            println!("Command output: {:?}", output);

            // Grab the output paths from the script to update data_paths
            data_paths = re.captures_iter(&output.stdout).map(|c| String::from_utf8(c[1].to_vec()).unwrap()).collect();
            println!("Output paths: {:?}", data_paths);
            println!("Captures: {:?}", re.captures_iter(&output.stdout).collect::<Vec<_>>());

            // Restore the repo to the state it was in before running the script
            if using_git && checkout_res.is_some() {
                // TODO: Checking out and restoring the repo is something RAII should be able to do
                let (did_restore, head) = checkout_res.unwrap();
                utils::safe_git_checkout_commit_restore(repo.as_mut().unwrap(), did_restore, &head)?;
            }
        }

        let new_data = Data {
            id: id,
            md: HashMap::new(),
            paths: data_paths
        };
        Ok(new_data)
    }
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