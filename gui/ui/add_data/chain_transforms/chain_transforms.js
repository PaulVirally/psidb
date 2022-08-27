(function () { // Wrap everything in a closure
    async function try_chain_transforms() {
        if (!await is_db_loaded()) {
            return;
        }

        const [transform_ids, data_ids_ok] = await get_ids("ids");
        const [md, md_ok] = await get_md();
        const ok = data_ids_ok && md_ok;
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_link = await invoke("chain", {transformIds: transform_ids, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_link) {
            message("Error: Failed to chain transforms. Maybe the IDs provided do not exist or the new transform already exists?", {type: "error"});
        } else {
            message("Transforms chained successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("chain-transform-btn");
    btn.addEventListener("click", try_chain_transforms);
})();