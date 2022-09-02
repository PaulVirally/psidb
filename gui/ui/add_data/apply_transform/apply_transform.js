(function () { // Wrap everything in a closure
    async function try_chain_transforms() {
        if (!await is_db_loaded()) {
            return;
        }

        const [transform_id, transform_id_ok] = await get_ids("transform-ids");
        const [data_ids, data_ids_ok] = await get_ids("ids", false, "Warning: No data IDs specified");
        const [md, md_ok] = await get_md();
        const ok = transform_id_ok && data_ids_ok && md_ok && transform_id.length === 1;
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_link = await invoke("apply", {transformId: transform_id[0], dataIds: data_ids, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_link) {
            message("Error: Failed to apply transform transforms. Maybe the ID provided does not exist or the new data already exists?", {type: "error"});
        } else {
            message("Transform applied successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("apply-transform-btn");
    btn.addEventListener("click", try_chain_transforms);
})();