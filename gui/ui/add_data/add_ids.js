let add_id = function (btn_id, div_id) {
    let id_idx = 0;

    function remove_id(idx) {
        const container = document.getElementById("ids");
        const elem = Array.from(container.children).find(elem => elem.getAttribute("idx") == idx.toString());
        elem.remove();
    }

    function create_id_entry(idx) {
        const id_input = document.createElement("input");
        id_input.setAttribute("name", "id");
        id_input.setAttribute("type", "number");
        id_input.setAttribute("placeholder", "ID");
        const btn = document.createElement("button");
        btn.innerText = "Remove";
        btn.addEventListener("click", () => {remove_id(idx);});

        const container = document.createElement("div");
        container.setAttribute("idx", idx.toString());
        container.appendChild(id_input);
        container.appendChild(btn);

        return container;
    }

    function add_ids(div_id) {
        const container = document.getElementById(div_id);
        const entry = create_id_entry(id_idx++);
        container.appendChild(entry);
    }

    const add_id_btn = document.getElementById(btn_id);
    add_id_btn.addEventListener("click", () => add_ids(div_id));
}