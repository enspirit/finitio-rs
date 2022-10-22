use finitio::fio;
use finitio::schema;
use finitio::schema::Schema;
use finitio::schema::errors::ValidationError;
use snafu::ErrorCompat;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use std::fs;
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
            match parse(schema) {
                Ok(_) => println!("Your schema is valid!"),
                Err(err) => panic!("Your schema is invalid: {}", err),
            }
            Ok(())
        },
        Commands::Validate { json, schema, r#type } => {
            // Parse FIO file
            let schema = match parse(schema) {
                Ok(schema) => schema,
                Err(err) => panic!("Your schema is invalid: {}", err),
            };

            let data = load_json(json)?;

            let target = schema.types.get(r#type);
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

pub fn parse(filename: &String) -> Result<Schema, ValidationError> {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut fios: Vec<fio::Schema> = Vec::new();

    // Parse entry point
    let main_fio = fio::parse_schema(&contents[..]).expect("Syntax error");
    fios.push(main_fio);

    // Parse imports
    if !fios[0].imports.is_empty() {
        let base_dir = Path::new(filename).parent()
            .expect("base_dir could not be determined from source");
        let mut included_files: HashSet<PathBuf> = HashSet::new();
        let mut includes = fios[0]
            .imports
            .iter()
            .map(|p| (base_dir.join(&p.filename)))
            .collect::<Vec<_>>();
        while !includes.is_empty() {
            let include = includes.remove(0);
            if included_files.contains(&include) {
                continue;
            }
            included_files.insert(include.clone());
            let contents = fs::read_to_string(include.clone())
                .expect("Should have been able to read the file");
            let fio = fio::parse_schema(&contents[..])
                .expect("Syntax error");
            let dir = include.parent().unwrap();
            includes.extend(fio.imports.iter().map(|inc| dir.join(&inc.filename)));
            fios.push(fio);
        }
    }

    schema::Schema::from_fio(fios.iter())
}
