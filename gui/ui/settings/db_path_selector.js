// Access Tauri
const invoke = window.__TAURI__.invoke;
const message = window.__TAURI__.dialog.message;
const open = window.__TAURI__.dialog.open;

async function choose_db_path() {
    let db_path = await open({
        directory: true,
        multiple: false,
        recursive: false,
        title: "Choose the directory containing the database (db.ron)"
    });
    if (Array.isArray(db_path) || db_path === null) {
        db_path = "";
    }
    const db_loaded = await invoke("load_db", {dbPath: db_path});
    if (!db_loaded) {
        message("The currently chosen directory does not contain a psidb database (db.ron file)", {type: "error"});
        return null;
    }
    return db_path;
}

async function update_db_path_selector(path=null) {
    if (path === null) {
        // Choose the path from the file system
        path = await choose_db_path();
    }
    if (path === null) {
        // If the path chosen did not have a db.ron file, make sure we still get the path that was chosen by the user
        path = await invoke("get_curr_psidb_dir");
    }
    document.getElementById("db-path-selector-result").innerText = path;
}

// Populate the db path selector with the default value of $HOME/.psidb
invoke("get_curr_psidb_dir").then((response) => update_db_path_selector(response));
document.getElementById("db-path-selector-btn").addEventListener("click", () => {update_db_path_selector(null)});