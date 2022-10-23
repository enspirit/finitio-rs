use finitio::fio::parse_file;
use finitio::js;
use finitio::schema;
use snafu::ErrorCompat;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        schema: String,
    },
    Validate {
        #[arg(short, long)]
        r#type: String,
        #[arg(short, long)]
        schema: String,
        #[arg(short, long)]
        json: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Parse { schema } => {
            // Parse FIO file
            let fios = match parse_file(&PathBuf::from(schema)) {
                Ok(fios) => fios,
                Err(err) => panic!("Your schema is invalid: {}", err),
            };
            js::generate_json(&fios)?;

            Ok(())
        },
        Commands::Validate { json, schema, r#type } => {
            let entry_path = PathBuf::from(schema);
            let fios = match parse_file(&entry_path) {
                Ok(schema) => schema,
                Err(err) => panic!("Your schema is invalid: {}", err),
            };

            let schemas = match schema::Schema::from_fios(fios) {
                Ok(schema) => schema,
                Err(e) => {
                    eprintln!("{}", e);
                    panic!();
                },
            };

            let entry_schema = schemas.get(&entry_path).expect("Found entry schema in map of validated schemas");
            let data = load_json(json)?;

            let target = entry_schema.types.get(r#type);
            match target {
                Some(t) => {
                    let t = t.to_owned();
                    match schema::TypeInclude::include(&t, &data) {
                        Ok(_) => println!("Valid data!"),
                        Err(e) => {
                            eprintln!("Invalid data: {}", e);
                            for cause in ErrorCompat::iter_chain(&e) {
                                eprintln!("due to: {}", cause);
                            }
                        }
                    }
                },
                None => panic!("Could not find the targetted type"),
            }
            Ok(())
        },
    }
}

pub fn load_json(filename: &String) -> Result<serde_json::Value, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
