(function () { // Wrap everything in a closure
    async function try_connect() {
        if (!await is_db_loaded()) {
            return;
        }

        const [in_data_ids, in_data_ids_ok] = await get_ids("in-data-ids", false, "");
        const [out_data_ids, out_data_ids_ok] = await get_ids("out-data-ids", false, "");
        const [in_transform_ids, in_transform_ids_ok] = await get_ids("in-transform-ids", false, "");
        const [out_transform_ids, out_transform_ids_ok] = await get_ids("out-transform-ids", false, "");
        const action = document.getElementById("action").value;
        console.log(action);
        const [md, md_ok] = await get_md();
        const ok = in_data_ids_ok && out_data_ids_ok && in_transform_ids_ok && out_transform_ids_ok && md_ok
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_link = await invoke("connect", {action: action, inDataIds: in_data_ids, outDataIds: out_data_ids, inTransformIds: in_transform_ids, outTransformIds: out_transform_ids, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_link) {
            message("Error: Failed to connect entries. Maybe the ID provided does not exist or the new entry already exists?", {type: "error"});
        } else {
            message("Connection created successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("connect-btn");
    btn.addEventListener("click", try_connect);
})();