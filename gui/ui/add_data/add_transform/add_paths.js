(function () {
    let path_idx = 0;

    function remove_path(idx) {
        const container = document.getElementById("paths");
        const elem = Array.from(container.children).find(elem => elem.getAttribute("idx") == idx.toString());
        elem.remove();
    }

    function create_path_entry(path, idx) {
        const p = document.createElement("p");
        p.innerText = path;

        const args_input = document.createElement("input");
        args_input.setAttribute("name", "args");
        args_input.setAttribute("type", "text");
        args_input.setAttribute("placeholder", "Script arguments");

        const btn = document.createElement("button");
        btn.innerText = "Remove";
        btn.addEventListener("click", () => {remove_path(idx);});

        const container = document.createElement("div");
        container.setAttribute("idx", idx.toString());
        container.appendChild(p);
        container.appendChild(args_input)
        container.appendChild(btn);

        return container;
    }

    async function add_paths() {
        const open = window.__TAURI__.dialog.open;

        // Get the paths from the user's file system
        const open_out = await open({
            multiple: true,
            title: "Choose the path(s)"
        });
        const paths = Array.isArray(open_out ?? []) ? (open_out ?? []) : [open_out]; // Turns out you *can* have fun writing frontend code :D

        // Add the elements that contain the path and the remove path button
        const container = document.getElementById("paths");
        const size = container.childElementCount;
        for (const [i, path] of paths.entries()) {
            const entry = create_path_entry(path, path_idx++);
            container.appendChild(entry);
        }
    }

    const path_btn = document.getElementById("add-path-btn");
    path_btn.addEventListener("click", add_paths);
})();