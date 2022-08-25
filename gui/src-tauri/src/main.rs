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
            init_db
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
