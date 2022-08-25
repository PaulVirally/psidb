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
        await message("Error: Please specify at least one path", {type: "error"});
    }

    return [paths, paths_ok];
}