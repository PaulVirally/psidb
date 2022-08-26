async function is_db_loaded() {
    const invoke = window.__TAURI__.invoke;
    const message = window.__TAURI__.dialog.message;

    is_loaded = await invoke("is_db_loaded");
    if (!is_loaded) {
        await message("No database loaded. Try initializing one or go to the settings to specify the database location");
    }
    return is_loaded;
}