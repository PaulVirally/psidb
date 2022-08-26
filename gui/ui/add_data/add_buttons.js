(async function () {
    function create_button(inner_text, event_listener) {
        let btn = document.createElement("button");
        btn.innerText = inner_text;
        btn.addEventListener("click", event_listener);
        return btn;
    }
    
    function goto(location) {
        window.location.href = location;
    }
    
    add_data = () => {goto("/add_data/add_data/index.html")};
    add_transform = () => {goto("/add_data/add_transform/index.html")};
    chain_transforms = () => {goto("/add_data/chain_transforms/index.html")};
    link_data = () => {goto("/add_data/link_data/index.html")};
    apply_transform = () => {goto("/add_data/apply_transform/index.html")};
    init_db = () => {goto("/add_data/init_db/index.html")};
    connect = () => {goto("/add_data/connect/index.html")};
    
    const container = document.getElementById("btn-container");
    if (await is_db_loaded()) {
        // Loaded the database, invite the user to add data
        const add_data_btn = create_button("Add Data", add_data);
        container.appendChild(add_data_btn);
        
        const add_transform_btn = create_button("Add Transform", add_transform);
        container.appendChild(add_transform_btn);
        
        const chain_btn = create_button("Chain Transforms", chain_transforms);
        container.appendChild(chain_btn);
        
        const link_btn = create_button("Link Datasets", link_data);
        container.appendChild(link_btn);
        
        const apply_btn = create_button("Apply Transform", apply_transform);
        container.appendChild(apply_btn);
        
        const connect_btn = create_button("Connect Entries", connect);
        container.appendChild(connect_btn);
    }
    else {
        // Could not load the database, warn the user and invite them to initialize the database
        const warning_p = document.createElement("p");
        warning_p.innerText = "Could not find a database, try initializing one or go to settings to specify the databse location";
        container.appendChild(warning_p);
        
        const init_btn = create_button("Initialize Database", init_db);
        container.appendChild(init_btn);
    }
})();