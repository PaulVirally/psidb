(function () { // Wrap everything in a closure
    async function try_add_data() {
        if (!await is_db_loaded()) {
            return;
        }

        const [paths, paths_ok] = await get_paths();
        const [md, md_ok] = await get_md();
        const ok = paths_ok && md_ok;
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_add = await invoke("add_data", {dataPaths: paths, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_add) {
            message("Error: Failed to add data. Maybe the data already exists", {type: "error"});
        } else {
            message("Data added successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("add-data-btn");
    btn.addEventListener("click", try_add_data);
})();