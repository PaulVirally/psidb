(function () { // Wrap everything in a closure
    async function try_link_data() {
        if (!await is_db_loaded()) {
            return;
        }

        const [data_ids, data_ids_ok] = await get_ids("ids");
        const [md, md_ok] = await get_md();
        const ok = data_ids_ok && md_ok;
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_link = await invoke("link", {dataIds: data_ids, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_link) {
            message("Error: Failed to link datasets. Maybe the IDs provided do not exist or the new dataset already exists?", {type: "error"});
        } else {
            message("Data linked successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("link-data-btn");
    btn.addEventListener("click", try_link_data);
})();