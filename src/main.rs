mod utils;
mod database;
use clap::{Args, ArgGroup, Parser, Subcommand};
use database::{Database, entry::action::Action};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(Init),
    AddData(AddData),
    AddTransform(AddTransform),
    Connect(Connect)
}

#[derive(Args)]
struct Init {
    #[clap(value_parser)]
    db_path: Option<String>,
}

#[derive(Args)]
struct AddData {
    #[clap(long = "db")]
    db_path: Option<String>,
    
    #[clap(long = "md")]
    meta_data: Option<String>,

    #[clap(value_parser)]
    data_paths: Vec<String>
}

#[derive(Args)]
struct AddTransform {
    #[clap(long = "db")]
    db_path: Option<String>,
    
    #[clap(long = "md")]
    meta_data: Option<String>,

    #[clap(value_parser)]
    script_paths: Vec<String>,

    #[clap(long = "args")]
    script_args: Option<String>,

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
    #[clap(long = "db")]
    db_path: Option<String>,

    #[clap(long = "md")]
    meta_data: Option<String>,

    #[clap(arg_enum, short, long)]
    action: Action,

    #[clap(long)]
    in_data_ids: Option<Vec<u64>>,

    #[clap(long)]
    out_data_ids: Option<Vec<u64>>,

    #[clap(long)]
    in_transform_ids: Option<Vec<u64>>,

    #[clap(long)]
    out_transform_ids: Option<Vec<u64>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Init(Init{db_path}) => {
            Database::init(db_path)?;
        }
        Commands::AddData(AddData{db_path, meta_data, data_paths}) => {
            let mut db = Database::load(db_path)?;
            let id = db.add_data(data_paths, meta_data)?;
            db.write()?;
            println!("Added data with id {}", id.to_string());
        }
        Commands::AddTransform(AddTransform{db_path, meta_data, script_paths, script_args, script_git_hashes}) => {
            let mut db = Database::load(db_path)?;
            let id = db.add_transform(script_paths, script_args, script_git_hashes, meta_data)?;
            db.write()?;
            println!("Added transform with id {}", id.to_string());
        }
        Commands::Connect(Connect{db_path, meta_data, action, in_data_ids, out_data_ids, in_transform_ids, out_transform_ids}) => {
            let mut db = Database::load(db_path)?;
            let id = db.connect(action, in_data_ids, out_data_ids, in_transform_ids, out_transform_ids, meta_data)?;
            db.write()?;
            println!("Added a connection with id {}", id.to_string());
        }
    }

    Ok(())
}
