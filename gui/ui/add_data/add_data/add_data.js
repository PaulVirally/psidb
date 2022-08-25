add_data.add_data = function () {
    async function get_paths() {
        let paths = [];
        const container = document.getElementById("paths");
        for (const entry of Array.from(container.children)) {
            const p = Array.from(entry.children).find(elem => elem.tagName == "P");
            paths.push(p.innerText);
        }
        const paths_ok = paths.length > 0;
        if (!paths_ok) {
            const message = window.__TAURI__.dialog.message;
            await message("Error: Please specify at least one data path", {type: "error"});
        }

        return [paths, paths_ok];
    }

    async function get_md() {
        let kv_arr = [];
        let should_warn = false;
        let md_ok = true;
        const form = document.getElementById("md-form");
        for (const kv_container of Array.from(document.getElementsByClassName("kv-entry"))) {
            const elems = Array.from(kv_container.children);
            const key = elems.find(elem => elem.classList.contains("key")).value ?? "";
            const value = elems.find(elem => elem.classList.contains("value")).value ?? "";
            if (key === "" || value === "") {
                should_warn = true;
            }
            if (key !== "" || value !== "") {
                kv_arr.push([key, value]);
            }
        }

        if (should_warn) {
            const confirm = window.__TAURI__.dialog.confirm;
            md_ok = await confirm("Warning: At least one of the key and/or value pairs is empty", {type: "warning"});
        } else if (kv_arr.length === 0) {
            const confirm = window.__TAURI__.dialog.confirm;
            md_ok = await confirm("Warning: No metadata specified", {type: "warning"});
        }
        return [kv_arr, md_ok];
    }

    function kv_to_string(kv_arr) {
        return kv_arr.map(kv => kv.join("=")).join(";");
    }

    async function try_add_data() {
        const [paths, paths_ok] = await get_paths();
        const [kv_arr, md_ok] = await get_md();
        const ok = paths_ok && md_ok;
        if (!ok) {
            return;
        }
        const md = kv_to_string(kv_arr);
        const invoke = window.__TAURI__.invoke;
        const did_add = await invoke("add_data", {dataPaths: paths, metaDataStr: md});

        const message = window.__TAURI__.dialog.message;
        if (!did_add) {
            message("Error: Failed to add data. Maybe the data already exists or the database is not loaded (check in settings)?", {type: "error"});
        } else {
            message("Data added successfully!", {type: "info"});
        }
    }

    const btn = document.getElementById("add-data-btn");
    btn.addEventListener("click", try_add_data);
}();