async function get_ids(elem_id, error_on_empty=true, error_msg="Error: No IDs specified") {
    let should_warn = false;
    let ids = [];
    const container = document.getElementById(elem_id);
    for (const entry of Array.from(container.children)) {
        const elems = Array.from(entry.children);
        const id = elems.find(elem => elem.getAttribute("name") == "id").value;
        if (id === "") {
            should_warn = true;
        } else {
            ids.push(parseInt(id));
        }
    }

    let ids_ok = true;
    if (should_warn) {
        const confirm = window.__TAURI__.dialog.confirm;
        ids_ok = await confirm("Warning: At least one of the IDs is either empty or not a number", {type: "warning"});
    } else if (ids.length === 0) {
        const confirm = window.__TAURI__.dialog.confirm;
        if (error_on_empty) {
            const message = window.__TAURI__.dialog.message;
            await message(error_msg, {type: "error"});
            ids_ok = false;
        } else if (error_msg !== "") {
            const confirm = window.__TAURI__.dialog.confirm;
            ids_ok = await confirm(error_msg, {type: "warning"});
        }
    }

    return [ids, ids_ok];
}