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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .manage(Mutex::new(
            AppData {
                db_path: Database::get_psidb_dir(None).into_os_string().into_string().unwrap(),
                db: None
            }
        ))
        .invoke_handler(tauri::generate_handler![
            load_db,
            get_curr_psidb_dir])
        .run(tauri::generate_context!())?;
    Ok(())
}
