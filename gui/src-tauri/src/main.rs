#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::State;
use std::sync::Mutex;
use psidb_lib::database::Database;

struct AppData {
    db_path: String,
    db: Option<Database>
}
type AppState<'a> = State<'a, Mutex<AppData>>;

#[tauri::command]
fn load_db(state: AppState, db_path: &str) -> bool {
    let mut data = state.lock().unwrap();

    // Set the path and load the new database
    data.db_path = if db_path.is_empty() {
        Database::get_psidb_dir(None).into_os_string().into_string().unwrap()
    } else {
        db_path.to_owned()
    };
    let db = Database::load(Some(&data.db_path));
    if db.is_err() {
        println!("Could not load database at {:#?}", db_path);
        data.db = None; // Unload the database if the path provided is not valid
        return false;
    }
    data.db = Some(db.unwrap());
    true
}

#[tauri::command]
fn get_curr_psidb_dir(state: AppState) -> String {
    let data = state.lock().unwrap();
    data.db_path.clone()
}

#[tauri::command]
fn is_db_loaded(state: AppState) -> bool {
    let data = state.lock().unwrap();
    data.db.is_some()
}

#[tauri::command]
fn init_db(state: AppState, db_path: &str) -> bool {
    let mut data = state.lock().unwrap();

    let passed = Database::init(Some(db_path)).is_ok();
    if passed {
        // Try to load the data
        let db = Database::load(Some(db_path));
        if db.is_err() {
            // Unload the database and return false if we could not load the database after initializing it
            data.db_path = db_path.to_string();
            data.db = None;
            return false;
        }
        data.db = Some(db.unwrap());
    }
    data.db_path = data.db.as_ref().unwrap().get_db_path(); // Update the path

    passed
}

#[tauri::command]
fn add_data(state: AppState, data_paths: Vec<&str>, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }

    let db = data.db.as_mut().unwrap();
    if db.add_data(&data_paths, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

#[tauri::command]
fn add_transform(state: AppState, script_paths: Vec<&str>, script_args_str: &str, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }
    let db = data.db.as_mut().unwrap();
    if db.add_transform(&script_paths, Some(script_args_str), None, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

#[tauri::command]
fn link(state: AppState, data_ids: Vec<u64>, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }
    let db = data.db.as_mut().unwrap();
    if db.link(&data_ids, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

#[tauri::command]
fn chain(state: AppState, transform_ids: Vec<u64>, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }
    let db = data.db.as_mut().unwrap();
    if db.chain(&transform_ids, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

#[tauri::command]
fn apply(state: AppState, transform_id: u64, data_ids: Vec<u64>, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }
    let db = data.db.as_mut().unwrap();
    if db.apply(transform_id, &data_ids, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

#[tauri::command]
fn connect(state: AppState, action: &str, in_data_ids: Vec<u64>, out_data_ids: Vec<u64>, in_transform_ids: Vec<u64>, out_transform_ids: Vec<u64>, meta_data_str: &str) -> bool {
    let mut data = state.lock().unwrap();

    if data.db.is_none() {
        return false;
    }
    let db = data.db.as_mut().unwrap();

    let action = match action {
        "Apply" => Some(psidb_lib::database::entry::action::Action::Apply),
        "Chain" => Some(psidb_lib::database::entry::action::Action::Chain),
        "Link" => Some(psidb_lib::database::entry::action::Action::Link),
        _ => None
    };
    if action.is_none() {
        return false;
    }
    let action = action.unwrap();

    // Convert the input data to Options (None is the vector is empty, Some(vec) otherwise)
    let in_data_ids = if in_data_ids.is_empty() {
        None
    } else {
        Some(in_data_ids.as_slice())
    };
    let out_data_ids = if out_data_ids.is_empty() {
        None
    } else {
        Some(out_data_ids.as_slice())
    };
    let in_transform_ids = if in_transform_ids.is_empty() {
        None
    } else {
        Some(in_transform_ids.as_slice())
    };
    let out_transform_ids = if out_transform_ids.is_empty() {
        None
    } else {
        Some(out_transform_ids.as_slice())
    };

    if db.connect(action, in_data_ids, out_data_ids, in_transform_ids, out_transform_ids, Some(meta_data_str)).is_err() {
        return false;
    }
    db.write().is_ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = Database::get_psidb_dir(None).into_os_string().into_string().unwrap();
    let db = if let Ok(db) = Database::load(None) {
        Some(db)
    } else {
        None
    };

    tauri::Builder::default()
        .manage(Mutex::new(
            AppData {
                db_path,
                db
            }
        ))
        .invoke_handler(tauri::generate_handler![
            load_db,
            get_curr_psidb_dir,
            is_db_loaded,
            init_db,
            add_data,
            add_transform,
            link,
            chain,
            apply,
            connect
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
