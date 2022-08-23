use clap::{Args, ArgGroup, Parser, Subcommand};
use psidb_lib::database::{Database, entry::action::Action};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database
    Init(Init),
    /// Add a number of data files to the database
    AddData(AddData),
    /// Add a transform (a script that modifies data) to the database
    AddTransform(AddTransform),
    /// Connect multiple transforms/data sets together
    Connect(Connect),
    /// Apply a tranform to a dataset, creating a new dataset
    Apply(Apply),
    /// Chain multiple datasets together, creating a new dataset
    Chain(Chain),
    /// Link multiple transforms together, creating a new transform
    Link(Link)
}

#[derive(Args)]
struct Init {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(value_parser)]
    db_path: Option<String>,
}

#[derive(Args)]
struct AddData {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,
    
    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The paths to the data that make up a dataset
    #[clap(value_parser)]
    data_paths: Vec<String>
}

#[derive(Args)]
struct AddTransform {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,
    
    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The paths to the scripts that make up a transform
    #[clap(value_parser)]
    script_paths: Vec<String>,

    /// The arguments passed to each script, one string per script
    #[clap(long = "args")]
    script_args: Option<String>,

    /// The commits of the versions of the scripts if they are tracked by git, one string per script
    #[clap(long = "hashes")]
    script_git_hashes: Option<String>
}

#[derive(Args)]
#[clap(group(
    ArgGroup::new("ids")
        .multiple(true)
        .required(true)
        .args(&["in-data-ids", "out-data-ids", "in-transform-ids", "out-transform-ids"])
))]
struct Connect {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,

    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The action of the connection, one of apply, chain, or link
    #[clap(arg_enum, short, long)]
    action: Action,

    /// The ids of the datasets forming the input to the connection
    #[clap(long)]
    in_data_ids: Option<Vec<u64>>,

    /// The ids of the datasets forming the output of the connection
    #[clap(long)]
    out_data_ids: Option<Vec<u64>>,

    /// The ids of the transforms forming the input to the connection
    #[clap(long)]
    in_transform_ids: Option<Vec<u64>>,

    /// The ids of the transforms forming the output of the connection
    #[clap(long)]
    out_transform_ids: Option<Vec<u64>>,
}

#[derive(Args)]
struct Apply {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,

    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The id of the transform to apply
    #[clap(short, long)]
    transform_id: u64,

    /// The datasets to apply the transform to
    #[clap(short, long)]
    data_ids: Vec<u64>
}

#[derive(Args)]
struct Chain {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,

    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The ids of the transforms to chain together
    #[clap(short, long)]
    transform_ids: Vec<u64>
}

#[derive(Args)]
struct Link {
    /// Path to the database folder, defaults to $HOME/.psidb/
    #[clap(long = "db")]
    db_path: Option<String>,

    /// The metadata associated with this dataset
    #[clap(long = "md")]
    meta_data: Option<String>,

    /// The ids of the datasets to link together
    #[clap(short, long)]
    data_ids: Vec<u64>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Init(Init{db_path}) => {
            Database::init(db_path.as_deref())?;
        }
        Commands::AddData(AddData{db_path, meta_data, data_paths}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let id = db.add_data(&data_paths, meta_data.as_deref())?;
            db.write()?;
            println!("Added data with id {}", id);
        }
        Commands::AddTransform(AddTransform{db_path, meta_data, script_paths, script_args, script_git_hashes}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let id = db.add_transform(&script_paths, script_args.as_deref(), script_git_hashes.as_deref(), meta_data.as_deref())?;
            db.write()?;
            println!("Added transform with id {}", id);
        }
        Commands::Connect(Connect{db_path, meta_data, action, in_data_ids, out_data_ids, in_transform_ids, out_transform_ids}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let id = db.connect(action, in_data_ids.as_deref(), out_data_ids.as_deref(), in_transform_ids.as_deref(), out_transform_ids.as_deref(), meta_data.as_deref())?;
            db.write()?;
            println!("Added a connection with id {}", id);
        }
        Commands::Apply(Apply{db_path, meta_data, transform_id, data_ids}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let (data_id, connect_id) = db.apply(transform_id, &data_ids, meta_data.as_deref())?;
            db.write()?;
            println!("Added data with id {} and connection with id {}", data_id, connect_id);
        }
        Commands::Chain(Chain{db_path, meta_data, transform_ids}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let (transform_id, connect_id) = db.chain(&transform_ids, meta_data.as_deref())?;
            db.write()?;
            println!("Added transform with id {} and connection with id {}", transform_id, connect_id);
        }
        Commands::Link(Link{db_path, meta_data, data_ids}) => {
            let mut db = Database::load(db_path.as_deref())?;
            let (data_id, connect_id) = db.link(&data_ids, meta_data.as_deref())?;
            db.write()?;
            println!("Added data with id {} and connection with id {}", data_id, connect_id);
        }
    }

    Ok(())
}
