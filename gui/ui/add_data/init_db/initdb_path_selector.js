// Access Tauri
const invoke = window.__TAURI__.invoke;
const message = window.__TAURI__.dialog.message;
const open = window.__TAURI__.dialog.open;

async function choose_db_path() {
    let db_path = await open({
        defaultPath: "/Users/pvirally/Desktop/does/not/exist/",
        directory: true,
        multiple: false,
        recursive: false,
        title: "Choose the directory to contain the database"
    });
    if (Array.isArray(db_path) || db_path === null) {
        db_path = "";
    }
    return db_path;
}

async function update_db_path_selector() {
    // Choose the path from the file system
    let path = await choose_db_path();
    const did_init = await invoke("init_db", {dbPath: path});
    path = await invoke("get_curr_psidb_dir"); // choose_db_path can return the wrong path if you close the window for example
    
    // Update the page to tell the user if the database was successfully initialized
    document.getElementById("initdb-path-selector-result").innerText = path + (did_init ? " ✅ Initialized successfully" : " ❌ Failed to initialize, try again");
}

document.getElementById("initdb-path-selector-btn").addEventListener("click", () => {update_db_path_selector(null)});