pub mod entry;
use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ron::ser::{PrettyConfig, to_writer_pretty};
use chrono::{Utc, SecondsFormat};
use entry::{data::Data, transform::Transform, connection::Connection, action::Action};
use super::utils;
use entry::{entry_in, id_in};

#[derive(Serialize, Deserialize)]
pub struct Database {
    db_path: String,
    data_vec: Vec<Data>,
    transform_vec: Vec<Transform>,
    connection_vec: Vec<Connection>,
    curr_id: u64
}

impl Database {
    pub fn new(path_str: Option<&str>) -> Result<Database, Box<dyn Error>> {
        let db_dir = Self::get_psidb_dir(path_str);
        fs::DirBuilder::new().recursive(true).create(&db_dir).unwrap();

        let db_path = db_dir.join("db.ron");

        // Check if db.ron already exists
        if db_path.exists() {
            println!("Error: {} already exists", db_path.to_str().unwrap());
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("{} already exists", db_path.to_str().unwrap()))));
        }

        Ok(Database {
            db_path: db_path.to_str().unwrap().to_owned(),
            data_vec: Vec::new(),
            transform_vec: Vec::new(),
            connection_vec: Vec::new(),
            curr_id: 0
        })
    }

    pub fn init(path_str: Option<&str>) -> Result<(), Box<dyn Error>> {
        let db = Database::new(path_str)?;
        db.write()?;
        println!("Created database at {}", db.db_path);
        Ok(())
    }

    pub fn load(path_str: Option<&str>) -> Result<Database, Box<dyn Error>> {
        let db_path = Self::get_psidb_dir(path_str).join("db.ron");

        // Check if db.ron exists
        if !db_path.exists() {
            println!("Error: The database does not exist. Create one with `psidb --init` or use the flag `--db <path>` to specify the location of the database.");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{} does not exist", db_path.to_str().unwrap()))));
        }

        let db_str = fs::read_to_string(&db_path)?;
        let db: Database = ron::from_str(&db_str)?;
        Ok(db)
    }

    fn get_psidb_dir(path_str: Option<&str>) -> PathBuf {
        let mut db_dir = if let Some(path_str) = path_str {
            PathBuf::from(&path_str).canonicalize().unwrap()
        } else {
            home::home_dir().unwrap_or_else(|| PathBuf::from("./")).canonicalize().unwrap()
        };
        // Make sure db_dir is a directory
        if !db_dir.as_path().is_dir() {
            db_dir = db_dir.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();
        }

        // The hidden directory .psidb stores the database
        db_dir.push(".psidb/");
        db_dir
    } 

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let serde_conf = PrettyConfig::new()
            .depth_limit(5)
            .indentor("\t".to_owned())
            .struct_names(true);

        let mut file = fs::File::create(&self.db_path)?;
        to_writer_pretty(&mut file, self, serde_conf)?;

        Ok(())
    }

    fn parse_md(meta_data_str: Option<&str>) -> HashMap<String, String> {
        if meta_data_str.is_none() {
            return HashMap::new()
        }
        let mut md = HashMap::new();
        for s in meta_data_str.unwrap().split(';') {
            let args: Vec<&str> = s.split('=').collect();
            md.insert(args[0].to_owned(), args[1..].join("="));
        };
        md
    }

    fn try_add_data(&mut self, mut data: Data) -> Result<u64, Box<dyn Error>> {
        // Make sure the data is not empty
        if data.paths.is_empty() {
            println!("Error: No data paths");
            return Err("No data paths".into());
        }

        // Check if the data already exists in the database
        match entry_in(&data, &self.data_vec) {
            (true, id) => {
                println!("Error: Data with the same paths already exists at id {}", id);
                return Err(format!("Data with the same paths already exists at id {}", id).into());
            }
            (false, ..) => {}
        }
        data.id = self.curr_id;

        // Add the data to the database
        self.data_vec.push(data);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn add_data<T> (&mut self, data_paths: &[T], meta_data_str: Option<&str>) -> Result<u64, Box<dyn Error>> 
    where T: AsRef<str> + AsRef<std::ffi::OsStr> + std::fmt::Display {
        // Check if the paths are valid and make the paths aboslute paths
        let mut used_paths: Vec<String> = vec!["".to_owned(); data_paths.len()];
        for (i, path) in data_paths.iter().enumerate() {
            if Path::new(path).exists() {
                used_paths[i] = Path::new(path).canonicalize()?.to_str().unwrap().to_owned();
            }
            else {
                println!("Error: Path {} does not exist", path);
                return Err(format!("Path {} does not exist", path).into());
            }
        }

        // Add the current time to the meta data if it doesn't already exist
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }

        // Construct the data
        let data = Data {
            id: self.curr_id,
            md,
            paths: used_paths
        };

        // Add the data to the database
        self.try_add_data(data)
    }

    fn try_add_transform(&mut self, mut transform: Transform) -> Result<u64, Box<dyn Error>> {
        // Make sure there is at least one script path
        if transform.script_paths.is_empty() {
            println!("Error: No script paths");
            return Err("No script paths".into());
        }

        // Check if the transform already exists in the database
        match entry_in(&transform, &self.transform_vec) {
            (true, id) => {
                println!("Error: Transform with the same name already exists at id {}", id);
                return Err(format!("Transform with the same name already exists at id {}", id).into());
            }
            (false, ..) => {}
        }
        transform.id = self.curr_id;

        // Add the transform to the database
        self.transform_vec.push(transform);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn add_transform<T>(&mut self, script_paths: &[T], script_args_str: Option<&str>, script_git_hashes_str: Option<&str>, meta_data_str: Option<&str>) -> Result<u64, Box<dyn Error>>
    where T: AsRef<str> + AsRef<std::ffi::OsStr> + std::fmt::Display {
        let script_args = utils::parse_kv_opt_string(script_args_str, Some(script_paths.len()));
        let script_git_hashes = utils::parse_kv_opt_string(script_git_hashes_str, Some(script_paths.len()));

        // Make sure that the script_paths, script_args, and script_git_hashes are the same length
        if script_paths.len() != script_args.len() || script_paths.len() != script_git_hashes.len() {
            println!("Error: script_paths, script_args, and script_git_hashes must be the same length");
            return Err("script_paths, script_args, and script_git_hashes must be the same length".into());
        }

        // Get the hashes of the scripts if they are not provided (or None if they are not tracked by git), and get the absolute paths of the scripts
        let mut used_hashes: Vec<Option<String>> = vec![None; script_paths.len()];
        let mut used_paths: Vec<String> = vec!["".to_owned(); script_paths.len()];
        for (i, (path_str, hash)) in script_paths.iter().zip(script_git_hashes).enumerate() {
            // Make sure the script exists and is a file
            utils::verify_file_path(path_str)?;

            // Get the absolute path of the script
            let path = Path::new(path_str);
            used_paths[i] = path.canonicalize()?.to_str().unwrap().to_owned();

            // Check if the file is in a git repository
            let parent_dir = path.parent().unwrap_or_else(|| Path::new("/"));
            let repo = git2::Repository::discover(parent_dir);
            if repo.is_err() {
                used_hashes[i] = None;
                continue;
            }
            let repo = repo?;

            if let Some(hash) = hash {
                // Check if the hash is a valid commit
                let oid = git2::Oid::from_str(hash.as_str())?;
                let commit = repo.find_commit(oid);
                if commit.is_err() {
                    // TODO: Should this be a fatal error instead of a warning?
                    println!("Warning: Ignoring hash {} for {} because it is not a valid commit", hash, path_str);
                }
                else {
                    used_hashes[i] = Some(hash.to_owned());
                    continue;
                }
            }

            // Get the hash of the latest commit
            let target = repo.head()?.target();
            if let Some(target) = target {
                // If the repository has a head, get the hash of the latest commit
                used_hashes[i] = Some(target.to_string());
                continue;
            }
            used_hashes[i] = None;
        }

        // Add the current time to the meta data if it doesn't already exist
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }

        let transform = Transform {
            id: self.curr_id,
            md,
            script_paths: used_paths,
            script_args,
            script_git_hashes: used_hashes
        };

        // Add the transform
        self.try_add_transform(transform)
    }

    pub fn connect(&mut self, action: Action, in_data_ids: Option<&[u64]>, out_data_ids: Option<&[u64]>, in_transform_ids: Option<&[u64]>, out_transform_ids: Option<&[u64]>, meta_data_str: Option<&str>) -> Result<u64, Box<dyn Error>> {
        // Make sure we have at least one data id or one transform id
        if in_data_ids.is_none() && out_data_ids.is_none() && in_transform_ids.is_none() && out_transform_ids.is_none() {
            println!("Error: Must provide at least one data id or transform id");
            return Err("Must provide at least one data id or transform id".into());
        }

        let in_data_ids = in_data_ids.unwrap_or(&[]);
        let out_data_ids = out_data_ids.unwrap_or(&[]);
        let in_transform_ids = in_transform_ids.unwrap_or(&[]);
        let out_transform_ids = out_transform_ids.unwrap_or(&[]);
        
        // Check to see if all the data_ids exist in the database
        for id in in_data_ids {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Input data with id {} does not exist", id);
                return Err(format!("Input data with id {} does not exist", id).into());
            }
        }
        for id in out_data_ids {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Output data with id {} does not exist", id);
                return Err(format!("Output data with id {} does not exist", id).into());
            }
        }

        // Check to see if all the transform_ids exist in the database
        for id in in_transform_ids {
            if !id_in(*id, &self.transform_vec) {
                println!("Error: Input transform with id {} does not exist", id);
                return Err(format!("Input transform with id {} does not exist", id).into());
            }
        }
        for id in out_transform_ids {
            if !id_in(*id, &self.transform_vec) {
                println!("Error: Output transform with id {} does not exist", id);
                return Err(format!("Output transform with id {} does not exist", id).into());
            }
        }

        // Add the current time to the meta data if it doesn't already exist
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }

        let connection = Connection {
            id: self.curr_id,
            md,
            action,
            in_data_ids: in_data_ids.to_vec(),
            out_data_ids: out_data_ids.to_vec(),
            in_transform_ids: in_transform_ids.to_vec(),
            out_transform_ids: out_transform_ids.to_vec()
        };

        // Check if the connection already exists in the database
        match entry_in(&connection, &self.connection_vec) {
            (true, id) => {
                println!("Error: Connection with the same action, data, and transforms already exists at id {}", id);
                return Err(format!("Connection with the same action, data and transforms already exists at id {}", id).into());
            }
            (false, ..) => {}
        }

        self.connection_vec.push(connection);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn apply(&mut self, transform_id: u64, data_ids: &[u64], meta_data_str: Option<&str>) -> Result<(u64, u64), Box<dyn Error>> {
        // Check if the transform exists
        if !id_in(transform_id, &self.transform_vec) {
            println!("Error: Transform with id {} does not exist", transform_id);
            return Err(format!("Transform with id {} does not eixst", transform_id).into());
        }

        // Check if the data ids exist
        for id in data_ids {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Data with id {} does not exist", *id);
                return Err(format!("Data with id {} does not eixst", *id).into());
            }
        }

        // Get the transform
        let transform = self.transform_vec.iter().find(|t| t.id == transform_id).unwrap();
        
        // Get the data
        let mut data = Vec::with_capacity(data_ids.len());
        for id in data_ids {
            data.push(self.data_vec.iter().find(|d| d.id == *id).unwrap());
        }

        // Apply the scripts in the transform sequentially
        let mut new_data = transform.apply(&data, self.curr_id)?;

        // Add the current time to the meta data if it doesn't already exist
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }
        new_data.md = md;

        // Add the data to the database
        let new_data_id = self.try_add_data(new_data)?;

        // Connect the new data to the transform
        let new_connect_id = self.connect(Action::Apply, Some(data_ids), Some(&[new_data_id]), Some(&[transform_id]), None, meta_data_str)?;

        Ok((new_data_id, new_connect_id))
    }

    pub fn chain(&mut self, transform_ids: &[u64], meta_data_str: Option<&str>) -> Result<(u64, u64), Box<dyn Error>> {
        // Check if all the transforms exist
        for id in transform_ids {
            if !id_in(*id, &self.transform_vec) {
                println!("Error: Transform with id {} does not exist", *id);
                return Err(format!("Transform with id {} does not eixst", *id).into());
            }
        }
        
        // Get the transforms
        let transforms = transform_ids
            .iter()
            .map(|id| self.transform_vec.iter().find(|t| t.id == *id)
            .unwrap())
            .collect::<Vec<&Transform>>();

        // The metadata for this new transform
        let mut given_md = Self::parse_md(meta_data_str);
        if !given_md.contains_key("time") {
            given_md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }

        let mut md = HashMap::new();
        let mut script_paths = vec![]; 
        let mut script_args = vec![];
        let mut script_git_hashes = vec![];
        for transform in &transforms {
            md.extend(transform.md.iter().map(|(k, v)| (k.clone(), v.clone())));
            script_paths.extend(transform.script_paths.iter().cloned());
            script_args.extend(transform.script_args.iter().cloned());
            script_git_hashes.extend(transform.script_git_hashes.iter().cloned());
        }
        md.extend(given_md);

        // Create the new transform
        let transform = Transform {
            id: self.curr_id,
            md,
            script_paths,
            script_args,
            script_git_hashes
        };

        // Add the transform to the database
        let new_transform_id = self.try_add_transform(transform)?;

        // Connect the new data to the transform
        let new_connect_id = self.connect(Action::Chain, None, None, Some(transform_ids), Some(&[new_transform_id]), meta_data_str)?;

        Ok((new_transform_id, new_connect_id))
    }

    pub fn link(&mut self, data_ids: &[u64], meta_data_str: Option<&str>) -> Result<(u64, u64), Box<dyn Error>> {
        // Check if the data ids exist
        for id in data_ids {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Data with id {} does not exist", *id);
                return Err(format!("Data with id {} does not eixst", *id).into());
            }
        }
        
        // Get the data
        let all_data = data_ids
            .iter()
            .map(|id| self.data_vec.iter().find(|d| d.id == *id).unwrap())
            .collect::<Vec<&Data>>();
        
        // The metadata for this new transform
        let mut given_md = Self::parse_md(meta_data_str);
        if !given_md.contains_key("time") {
            given_md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true));
        }

        let mut md = HashMap::new();
        let mut paths = vec![];
        for data in &all_data {
            md.extend(data.md.iter().map(|(k, v)| (k.clone(), v.clone())));
            paths.extend(data.paths.iter().cloned());
        }
        md.extend(given_md);

        // Create the new data
        let data = Data {
            id: self.curr_id,
            md,
            paths
        };

        // Add the data to the database
        let new_data_id = self.try_add_data(data)?;

        // Connect the new data to the transform
        let new_connect_id = self.connect(Action::Link, Some(data_ids), Some(&[new_data_id]), None, None, meta_data_str)?;

        Ok((new_data_id, new_connect_id))
    }
}