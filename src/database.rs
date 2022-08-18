pub mod entry;
use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ron::ser::{PrettyConfig, to_writer_pretty};
use chrono::{Utc, SecondsFormat};
use home;
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
    pub fn new(path_str: Option<String>) -> Result<Database, Box<dyn Error>> {
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

    pub fn init(path_str: Option<String>) -> Result<(), Box<dyn Error>> {
        let db = Database::new(path_str)?;
        db.write()?;
        println!("Created database at {}", db.db_path);
        Ok(())
    }

    pub fn load(path_str: Option<String>) -> Result<Database, Box<dyn Error>> {
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

    fn get_psidb_dir(path_str: Option<String>) -> PathBuf {
        let mut db_dir = if path_str.is_none() {
            home::home_dir().unwrap_or(PathBuf::from("./")).canonicalize().unwrap()
        } else {
            PathBuf::from(&path_str.unwrap()).canonicalize().unwrap()
        };
        // Make sure db_dir is a directory
        if !db_dir.as_path().is_dir() {
            db_dir = db_dir.parent().unwrap_or(Path::new("/")).to_path_buf();
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

    fn parse_md(meta_data_str: Option<String>) -> HashMap<String, String> {
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

    pub fn add_data(&mut self, data_paths: Vec<String>, meta_data_str: Option<String>) -> Result<u64, Box<dyn Error>> {
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

        // Add the current time to the meta data if it doesn't already exisyy
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        // Construct the data
        let data = Data {
            id: self.curr_id,
            md: md,
            paths: used_paths
        };

        // Check if the data already exists in the database
        match entry_in(&data, &self.data_vec) {
            (true, id) => {
                println!("Error: Data with the same paths already exists at id {}", id);
                return Err(format!("Data with the same paths already exists at id {}", id).into());
            }
            (false, ..) => {}
        }

        // Add the data to the database
        self.data_vec.push(data);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn add_transform(&mut self, script_paths: Vec<String>, script_args_str: Option<String>, script_git_hashes_str: Option<String>, meta_data_str: Option<String>) -> Result<u64, Box<dyn Error>> {
        let script_args = utils::parse_kv_opt_string(script_args_str, Some(script_paths.len()));
        let script_git_hashes = utils::parse_kv_opt_string(script_git_hashes_str, Some(script_paths.len()));

        // Make sure that the script_paths, script_args, and script_git_hashes are the same length
        if script_paths.len() != script_args.len() || script_paths.len() != script_git_hashes.len() {
            println!("script_paths, script_args, and script_git_hashes must be the same length");
            return Err("script_paths, script_args, and script_git_hashes must be the same length".into());
        }

        // Get the hashes of the scripts if they are not provided (or None if they are not tracked by git), and get the absolute paths of the scripts
        let mut used_hashes: Vec<Option<String>> = vec![None; script_paths.len()];
        let mut used_paths: Vec<String> = vec!["".to_owned(); script_paths.len()];
        for (i, (path_str, hash)) in script_paths.iter().zip(script_git_hashes).enumerate() {
            let path = Path::new(path_str);

            // Make sure the script exists and is a file
            if !path.exists() {
                println!("Error: Script {} does not exist", path_str);
                return Err(format!("Script {} does not exist", path_str).into());
            }

            // Make sure the script is a file
            if !path.is_file() {
                println!("Error: Script {} is not a file", path_str);
                return Err(format!("Script {} is not a file", path_str).into());
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
                    // TODO: Should this be a fata error instead of a warning?
                    println!("Warning: Ignoring hash {} for {} because it is not a valid commit", hash, path_str);
                }
                else {
                    used_hashes[i] = Some(hash.to_owned());
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
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let transform = Transform {
            id: self.curr_id,
            md: md,
            script_paths: used_paths,
            script_args: script_args,
            script_git_hashes: used_hashes
        };

        // Check if the transform already exists in the database
        match entry_in(&transform, &self.transform_vec) {
            (true, id) => {
                println!("Error: Transform with the same scripts, arguments, and hashes already exists at id {}", id);
                return Err(format!("Transform with the same scripts, arguments, and hashes already exists at id {}", id).into());
            }
            (false, ..) => {}
        }

        self.transform_vec.push(transform);
        self.curr_id += 1;

        Ok(self.curr_id - 1)
    }

    pub fn connect(&mut self, action: Action, in_data_ids: Option<Vec<u64>>, out_data_ids: Option<Vec<u64>>, in_transform_ids: Option<Vec<u64>>, out_transform_ids: Option<Vec<u64>>, meta_data_str: Option<String>) -> Result<u64, Box<dyn Error>> {
        // Make sure we have at least one data id or one transform id
        if in_data_ids.is_none() && out_data_ids.is_none() && in_transform_ids.is_none() && out_transform_ids.is_none() {
            println!("Error: Must provide at least one data id or transform id");
            return Err("Must provide at least one data id or transform id".into());
        }

        let in_data_ids = in_data_ids.unwrap_or_else(|| vec![]);
        let out_data_ids = out_data_ids.unwrap_or_else(|| vec![]);
        let in_transform_ids = in_transform_ids.unwrap_or_else(|| vec![]);
        let out_transform_ids = out_transform_ids.unwrap_or_else(|| vec![]);
        
        // Check to see if all the data_ids exist in the database
        for id in in_data_ids.iter() {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Input data with id {} does not exist", id);
                return Err(format!("Input data with id {} does not exist", id).into());
            }
        }
        for id in out_data_ids.iter() {
            if !id_in(*id, &self.data_vec) {
                println!("Error: Output data with id {} does not exist", id);
                return Err(format!("Output data with id {} does not exist", id).into());
            }
        }

        // Check to see if all the transform_ids exist in the database
        for id in in_transform_ids.iter() {
            if !id_in(*id, &self.transform_vec) {
                println!("Error: Input transform with id {} does not exist", id);
                return Err(format!("Input transform with id {} does not exist", id).into());
            }
        }
        for id in out_transform_ids.iter() {
            if !id_in(*id, &self.transform_vec) {
                println!("Error: Output transform with id {} does not exist", id);
                return Err(format!("Output transform with id {} does not exist", id).into());
            }
        }

        // Add the current time to the meta data if it doesn't already exist
        let mut md = Self::parse_md(meta_data_str);
        if !md.contains_key("time") {
            md.insert("time".to_owned(), Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true).to_owned());
        }

        let connection = Connection {
            id: self.curr_id,
            md: md,
            action: action,
            in_data_ids: in_data_ids,
            out_data_ids: out_data_ids,
            in_transform_ids: in_transform_ids,
            out_transform_ids: out_transform_ids
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
}