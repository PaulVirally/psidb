(function () { // Wrap everything in a closure
    function get_script_args() {
        const container = document.getElementById("paths");

        args = [];
        for (const entry of Array.from(container.children)) {
            const input = Array.from(entry.children).find(elem => elem.tagName == "INPUT");
            console.log(input);
            console.log(entry);
            args.push(input.value);
        }
        return args.join(";");
    }

    async function try_add_transform() {
        if (!await is_db_loaded()) {
            return;
        }

        const [paths, paths_ok] = await get_paths();
        const args = get_script_args();
        const [md, md_ok] = await get_md();
        const ok = paths_ok && md_ok;
        if (!ok) {
            return;
        }
        const invoke = window.__TAURI__.invoke;
        const did_add = await invoke("add_transform", {scriptPaths: paths, scriptArgsStr: args, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_add) {
            message("Error: Failed to add transform. Maybe the transform already exists or the database is not loaded (check in settings)?", {type: "error"});
        } else {
            message("Transform added successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("add-transform-btn");
    btn.addEventListener("click", try_add_transform);
})();