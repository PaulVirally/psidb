add_data.md = function () { // Wrap everything in a closure
    function add_md_entry() {
        const form = document.getElementById("md-form");
        const idx = form.childElementCount;

        const key_input = document.createElement("input");
        key_input.setAttribute("name", "key" + idx.toString());
        key_input.setAttribute("type", "text");
        key_input.classList.add("key");

        const value_input = document.createElement("input");
        value_input.setAttribute("name", "value" + idx.toString());
        value_input.setAttribute("type", "text");
        value_input.classList.add("value");

        kv_container = document.createElement("div");
        kv_container.classList.add("kv-entry");
        kv_container.appendChild(key_input);
        kv_container.appendChild(value_input);
        form.appendChild(kv_container);
    }

    const entry_btn = document.getElementById("add-md-entry-btn");
    entry_btn.addEventListener("click", add_md_entry);
}();