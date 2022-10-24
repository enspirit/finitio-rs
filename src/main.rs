use finitio::fio::parse_file;
use finitio::json;
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
    /// Parses finitio schemas and outputs the adt in a json format
    Parse {
        #[arg(short, long)]
        /// The path to the entry point schema file (.fio)
        schema: String,
    },
    /// Validates json data agains a finitio type
    Validate {
        #[arg(short, long)]
        /// The type that should be used to dress the data
        r#type: String,
        #[arg(short, long)]
        /// The path to the entry point schema file (.fio)
        schema: String,
        #[arg(short, long)]
        /// the path to the json file to be validated
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
            json::generate_json(&fios)?;

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
                    match schema::FinitioType::include(&t, &data) {
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
