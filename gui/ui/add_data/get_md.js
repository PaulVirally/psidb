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

    // Convert the array to a string that psidb can parse
    const md = kv_arr.map(kv => kv.join("=")).join(";");
    return [md, md_ok];
}